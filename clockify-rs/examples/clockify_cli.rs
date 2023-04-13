use anyhow::{bail, Result};
use std::str::FromStr;

use clockify_rs::{Client, Config};

/// The type of resource that is specified
#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Debug)]
enum Resource {
    Workspace,
    Project,
}

impl FromStr for Resource {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "workspace" => Ok(Self::Workspace),
            "project" => Ok(Self::Project),
            _ => bail!("Unknown resource {}", s),
        }
    }
}

/// The command to execute
#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Debug)]
enum Command {
    /// Lists all projects, workspaces,...
    List,

    /// Gives detailed info about the resource
    Info,
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "list" => Ok(Self::List),
            "info" => Ok(Self::Info),
            _ => bail!("Unknown command {}", s),
        }
    }
}

struct Options {
    pub config: clockify_rs::Config,
    pub command: Command,
    pub resource: Resource,
    pub workspace_id: String,
    pub project_id: String,
}

/// Prints the usage of the CLI.
fn print_usage() {
    println!("clockify_cli <command> <resource> [<args>]\n\n");
    println!("There are the following commands available:");
    println!("list: List all objects of the specified resource");
    println!("info: Prints detailed info about the specified resource");
    println!("");
    println!("There are the following resource available:");
    println!("workspace: A workspace that potentially contains multiple projects");
    println!("project: Projects within a workspace. Needs to specify the workspace ID.");
}

/// Parses the API key from the environment variable API_KEY.
fn parse_api_key() -> Result<String> {
    let api_key = std::env::var("API_KEY")?;
    Ok(api_key)
}

/// Parses all arguments provided by the program arguments and environment variables.
fn parse_args() -> Result<Option<Options>> {
    let mut workspace_id = String::new();
    let mut project_id = String::new();

    let args: Vec<String> = std::env::args().collect();
    let args = &args[1..];

    if args.len() == 0 {
        print_usage();
        return Ok(None);
    } else if args.len() < 2 {
        bail!("Not enough arguments");
    }

    let command: Command = args[0].parse()?;
    let resource: Resource = args[1].parse()?;

    let api_key = parse_api_key()?;
    let config = Config::new(api_key);

    // determine the workspace id
    if command == Command::Info || resource != Resource::Workspace {
        if args.len() < 3 {
            bail!("Missing workspace ID");
        } else {
            workspace_id = args[2].clone();
        }

        // check if there is a project ID
        if command == Command::Info && resource == Resource::Project {
            if args.len() < 4 {
                bail!("Missing project ID");
            } else {
                project_id = args[3].clone();
            }
        }
    }

    Ok(Some(Options {
        command,
        resource,
        workspace_id,
        project_id,
        config,
    }))
}

async fn command_list(options: Options) -> Result<()> {
    let client = Client::new(options.config.clone()).await?;

    match options.resource {
        Resource::Workspace => {
            let workspaces = client.get_workspaces().await?;
            for workspace in workspaces.iter() {
                println!("ID={}, Name={}", workspace.id, workspace.name);
            }
        }
        Resource::Project => {
            let projects = client.get_projects(&options.workspace_id).await?;
            for project in projects.iter() {
                println!(
                    "ID={}, Name={}, Billable={}",
                    project.id, project.name, project.billable
                );
            }
        }
        _ => {
            bail!("Resource {:?} not implemented yet", options.resource);
        }
    }

    Ok(())
}

async fn command_info(options: Options) -> Result<()> {
    let client = Client::new(options.config.clone()).await?;

    match options.resource {
        Resource::Workspace => {
            let workspaces = client.get_workspaces().await?;
            match workspaces.iter().find(|w| w.id == options.workspace_id) {
                None => {
                    bail!("Cannot find workspace with ID '{}'", options.workspace_id);
                }
                Some(w) => {
                    println!("Workspace Info:\n");
                    println!("ID:\t{}", w.id);
                    println!("Name:\t{}", w.name);
                    println!("");
                }
            }
        }
        Resource::Project => {
            let project = client
                .get_project(&options.workspace_id, &options.project_id)
                .await?;

            println!("Project Info:\n");
            println!("ID:       {}", project.id);
            println!("ClientID: {}", project.client_id);
            println!("Name:     {}", project.name);
            println!("Billable: {}", project.billable);
            println!("Public:   {}", project.public);
            println!("Color:    {}", project.color);
            println!("Note:\n{}", project.note);

            println!("");
        }
        _ => {
            bail!("Resource {:?} not implemented yet", options.resource);
        }
    }

    Ok(())
}

/// Runs the program with the given program options
///
/// # Arguments
/// * `options` - The program options to run the program with.
async fn run_program(options: Options) -> Result<()> {
    match options.command {
        Command::List => command_list(options).await,
        Command::Info => command_info(options).await,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let options = match parse_args() {
        Ok(options) => match options {
            Some(options) => options,
            None => {
                std::process::exit(0);
            }
        },
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(-1);
        }
    };

    match run_program(options).await {
        Ok(()) => {
            println!("SUCCESS");
            Ok(())
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            eprintln!("FAILED");

            Err(err)
        }
    }
}
