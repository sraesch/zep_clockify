use anyhow::{bail, Result};
use enum_display_derive::Display;
use std::{fmt::Display, fs::File, path::Path, str::FromStr};

use struct_info::StructInfoDerive;

use crate::csv_deserialize::{deserialize_csv, StructInfo};

#[derive(Clone, Copy, Default, Debug, Display)]
pub enum Status {
    Active,

    #[default]
    Planning,
}

impl FromStr for Status {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "planning" => Ok(Self::Planning),
            _ => bail!("Unknown status {}", s),
        }
    }
}

#[derive(Default, Clone, StructInfoDerive)]
pub struct Project {
    #[StructInfoName = "ID"]
    pub id: u32,

    #[StructInfoName = "Status"]
    pub status: Status,

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
