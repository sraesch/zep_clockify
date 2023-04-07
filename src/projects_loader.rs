use std::{path::Path, fs::File};
use anyhow::Result;
use log::debug;

use crate::csv_parser::{CSVParser, CSVReader};

pub struct Project {
    name: String,
    description: String,
}

impl Project {

}

struct ProjectsReader {
    header: Vec<String>,
    projects: Vec<Project>,
}

impl ProjectsReader {
    pub fn new() -> Self {
        Self{
            projects: Vec::new(),
            header: Vec::new(),
        }
    }
}

impl CSVReader for ProjectsReader {
    fn header_record(&mut self, header: Vec<String>) {
        debug!("Received Header:");
        for (column_index, column) in header.iter().enumerate() {
            debug!("({}): {}", column_index + 1, column);
        }
        debug!("END");

        self.header = header;
        self.header.iter_mut().for_each(|h| *h = h.replace('\n', " "));
    }

    fn record(&mut self, record: &[String]) {
        debug!("Received Header:");
        for (column_index, (name, column)) in self.header.iter().zip(record.iter()).enumerate() {
            debug!("({}): {} = {}", column_index + 1, name, column);
        }
        debug!("END");
    }
}

pub fn load_projects(file_name: &Path) -> Result<Vec<Project>> {
    let file = File::open(file_name)?;
    let mut rdr = CSVParser::new(file);

    let mut project_reader = ProjectsReader::new();

    rdr.read(&mut project_reader)?;

    Ok(project_reader.projects)
}