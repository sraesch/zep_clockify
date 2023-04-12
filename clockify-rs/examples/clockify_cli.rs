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
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "list" => Ok(Self::List),
            _ => bail!("Unknown command {}", s),
        }
    }
}

struct Options {
    pub config: clockify_rs::Config,
    pub command: Command,
    pub resource: Resource,
    pub workspace_id: String,
}

/// Prints the usage of the CLI.
fn print_usage() {
    println!("clockify_cli <command> <resource> [<args>]\n\n");
    println!("There are the following commands available:");
    println!("list: List all objects of the specified resource");
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
    if command == Command::List && resource == Resource::Project {
        if args.len() < 3 {
            bail!("Missing workspace ID");
        } else {
            workspace_id = args[2].clone();
        }
    }

    Ok(Some(Options {
        command,
        resource,
        workspace_id,
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

/// Runs the program with the given program options
///
/// # Arguments
/// * `options` - The program options to run the program with.
async fn run_program(options: Options) -> Result<()> {
    match options.command {
        Command::List => command_list(options).await,
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
