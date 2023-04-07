mod projects_loader;
mod options;
mod csv_parser;

use std::path::Path;

use anyhow::{Result};
use log::{info, error};
use options::{Options, Operation};
use projects_loader::load_projects;

fn run_projects(csv_path: &Path) -> Result<()> {
    info!("Run Projects Import...");

    info!("Load input {}...", csv_path.to_string_lossy());
    let projects = load_projects(csv_path)?;

    Ok(())
}

fn run_program() -> Result<()> {
    let options = Options::parse_options()?;
    options.dump_to_log();

    match options.operation {
        Operation::Projects => run_projects(&options.csv_path)
    }
}

/// Initializes the program logging
fn initialize_logging() {
    simple_logging::log_to(std::io::stdout(), log::LevelFilter::Debug);
}

fn main() {
    initialize_logging();

    match run_program() {
        Ok(()) => info!("SUCCESS"),
        Err(err) => {
            error!("FAILED");
            error!("Error: {}", err);

            std::process::exit(-1);
        },
    }
}
