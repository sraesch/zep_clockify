use anyhow::{bail, Result};
use chrono::NaiveDate;
use enum_display_derive::Display;
use std::{fmt::Display, fs::File, path::Path, str::FromStr};

use struct_info::StructInfoDerive;

use crate::csv_deserialize::{deserialize_csv, CSVParsing, StructInfo};

#[derive(Clone, Copy, Default, Debug, Display)]
pub enum Status {
    Active,

    #[default]
    Planning,
}

impl CSVParsing for Status {
    fn csv_parse(&mut self, s: &str) -> Result<(), anyhow::Error> {
        match s {
            "active" => *self = Self::Active,
            "planning" => *self = Self::Planning,
            _ => bail!("Unknown status {}", s),
        }

        Ok(())
    }
}

impl CSVParsing for NaiveDate {
    fn csv_parse(&mut self, s: &str) -> Result<(), anyhow::Error> {
        // DD.MM.YYYY
        *self = NaiveDate::parse_from_str(s, "%d.%m.%Y")?;
        Ok(())
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

    #[StructInfoName = "Start date"]
    pub start_date: NaiveDate,
}

pub fn load_projects(file_name: &Path) -> Result<Vec<Project>> {
    let file = File::open(file_name)?;

    let projects: Vec<Project> = deserialize_csv(file)?;

    Ok(projects)
}
