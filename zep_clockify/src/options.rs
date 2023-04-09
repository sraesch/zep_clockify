use std::{path::PathBuf, str::FromStr, fmt::Display};

use anyhow::{Result, bail};
use log::info;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Operation {
    Projects,
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Projects => write!(f, "PROJECTS"),
        }
    }
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "PROJECTS" => Ok(Self::Projects),
            _ => {
                bail!("Unknown operation {}", s);
            }
        }
    }
}

pub struct Options {
    pub operation: Operation,
    pub csv_path: PathBuf,
}


impl Options {
    /// Parses the options from the program arguments.
    pub fn parse_options() -> Result<Self> {
        let args: Vec<String> = std::env::args().collect();
        if args.len() != 3 {
            Self::print_usage();
            bail!("Invalid arguments");
        }

        let operation = Operation::from_str(&args[1])?;
        let csv_path = PathBuf::from_str(&args[2])?;

        Ok(Self {
            operation,
            csv_path
        })
    }

    pub fn dump_to_log(&self) {
        info!("Operation: {}", self.operation);
        info!("CSV Input: {}", self.csv_path.to_string_lossy());
    }

    /// Prints the usage for the program.
    fn print_usage() {
        println!("zep_clockify COMMAND CSV\n");
        println!("COMMAND: The command to execute. Available commands are PROJECTS");
        println!("CSV: A path to a CSV file for reading the input");
    }
}