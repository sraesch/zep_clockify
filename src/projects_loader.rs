use std::{path::Path, fs::File};
use anyhow::Result;
use log::debug;

use crate::csv_parser::{CSVParser, ReaderResult};

pub struct Project {
    name: String,
    description: String,
}

impl Project {

fn print_header_record(header: &[String]) {
    debug!("Received Header:");
    for (column_index, column) in header.iter().enumerate() {
        debug!("({}): {}", column_index + 1, column);
    }
    debug!("END");
}

fn print_record(header: &[String], record: &[String]) {
    debug!("Received Header:");
    for (column_index, (name, column)) in header.iter().zip(record.iter()).enumerate() {
        debug!("({}): {} = {}", column_index + 1, name, column);
    }
    debug!("END");
}

pub fn load_projects(file_name: &Path) -> Result<Vec<Project>> {
    let file = File::open(file_name)?;
    let mut parser = CSVParser::new(file);

    let mut header = parser.read_header_record()?;
    header.iter_mut().for_each(|h| *h = h.replace('\n', " "));
    print_header_record(&header);

    let mut record = vec![String::new(); header.len()];

    loop {
        match parser.read_record(&mut record)? {
            ReaderResult::Data(_) => {
                print_record(&header, &record);
            }
            ReaderResult::Eof => break,
        }
    }
    Ok(Vec::new())
}
