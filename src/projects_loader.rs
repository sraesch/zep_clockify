use anyhow::Result;
use std::{fs::File, path::Path};

use crate::csv_deserialize::{deserialize_csv, StructInfo};

#[derive(Default, Clone)]
pub struct Project {
    id: u32,
    name: String,
    description: String,
}

impl StructInfo for Project {
    const NUM_FIELDS: usize = 3;

    fn get_field_name(index: usize) -> &'static str {
        match index {
            0 => "ID",
            1 => "Abbreviation",
            2 => "Description",
            _ => panic!("Index {} is out of range", index),
        }
    }

    fn parse_field(&mut self, index: usize, s: &str) -> Result<(), anyhow::Error> {
        match index {
            0 => {
                self.id = s.parse()?;
            }
            1 => {
                self.name = s.parse()?;
            }
            2 => {
                self.description = s.parse()?;
            }
            _ => panic!("Index {} is out of range", index),
        }

        Ok(())
    }
}

pub fn load_projects(file_name: &Path) -> Result<Vec<Project>> {
    let file = File::open(file_name)?;

    let projects: Vec<Project> = deserialize_csv(file)?;

    Ok(projects)
}
