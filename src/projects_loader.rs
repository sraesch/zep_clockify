use std::{path::Path, fs::File};
use anyhow::Result;

use crate::csv_parser::{CSVParser, CSVReader};

pub struct Project {
    name: String,
    description: String,
}

impl Project {

}

struct ProjectsReader {
    projects: Vec<Project>,
}

impl ProjectsReader {
    pub fn new() -> Self {
        Self{
            projects: Vec::new(),
        }
    }
}

impl CSVReader for ProjectsReader {
    fn header_record(&mut self, header: Vec<String>) {
        todo!()
    }

    fn record(&mut self, record: &[String]) {
        todo!()
    }
}

pub fn load_projects(file_name: &Path) -> Result<Vec<Project>> {
    let mut file = File::open(file_name)?;
    let mut rdr = CSVParser::new(file);

    let mut project_reader = ProjectsReader::new();

    rdr.read(&mut project_reader)?;

    todo!()

}