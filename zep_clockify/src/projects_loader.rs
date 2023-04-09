use anyhow::Result;
use std::{fs::File, path::Path};

use struct_info::StructInfoDerive;

use crate::csv_deserialize::{deserialize_csv, StructInfo};

#[derive(Default, Clone, StructInfoDerive)]
pub struct Project {
    #[StructInfoName = "ID"]
    pub id: u32,

    #[StructInfoName = "Abbreviation"]
    pub name: String,

    #[StructInfoName = "Description"]
    pub description: String,
}

pub fn load_projects(file_name: &Path) -> Result<Vec<Project>> {
    let file = File::open(file_name)?;

    let projects: Vec<Project> = deserialize_csv(file)?;

    Ok(projects)
}
