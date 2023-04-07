use std::{path::Path, fs::File};
use anyhow::Result;

pub struct Project {
    name: String,
    description: String,
}

impl Project {

}

pub fn load_projects(file_name: &Path) -> Result<Vec<Project>> {
    let mut file = File::open(file_name)?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }

    todo!()

}