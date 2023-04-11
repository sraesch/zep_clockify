use anyhow::{bail, Result};
use std::str::FromStr;

use clockify_rs::{self, Client, Config};

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
}

/// Prints the usage of the CLI.
fn print_usage() {
    println!("clockify_cli <command> <resource> [<args>]\n\n");
    println!("There are the following commands available:");
    println!("list: List all objects of the specified resource");
    println!("");
    println!("There are the following resource available:");
    println!("workspace: A workspace that potentially contains multiple projects");
    println!("project: Projects within a workspace");
}

/// Parses the API key from the environment variable API_KEY.
fn parse_api_key() -> Result<String> {
    let api_key = std::env::var("API_KEY")?;
    Ok(api_key)
}

/// Parses all arguments provided by the program arguments and environment variables.
fn parse_args() -> Result<Option<Options>> {
    let args: Vec<String> = std::env::args().collect();
    let args = &args[1..];

    if args.len() < 2 {
        print_usage();
        return Ok(None);
    }

    let command: Command = args[0].parse()?;
    let resource: Resource = args[1].parse()?;

    let api_key = parse_api_key()?;
    let config = Config::new(api_key);

    Ok(Some(Options {
        command,
        resource,
        config,
    }))
}

fn command_list(options: Options) -> Result<()> {
    let client = Client::new(options.config.clone());

    match options.resource {
        Resource::Workspace => {}
        _ => {
            bail!("Resource {:?} not implemented yet", options.resource);
        }
    }

    Ok(())
}

/// Runs the program
fn run_program(options: Options) -> Result<()> {
    match options.command {
        Command::List => command_list(options),
    }
}

pub fn main() {
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

    match run_program(options) {
        Ok(()) => {
            println!("SUCCESS");
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            eprintln!("FAILED");
        }
    }
}
