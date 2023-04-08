use std::io::Read;

use anyhow::{bail, Result};

use crate::csv_parser::{CSVParser, ReaderResult};

pub trait StructInfo: Clone + Default {
    const NUM_FIELDS: usize;

    /// Returns the name of the i-th field.
    ///
    /// # Arguments
    /// * `index` - The index of the field
    fn get_field_name(index: usize) -> &'static str;

    /// Returns a reference onto the i-th field.
    ///
    /// # Arguments
    /// * `index` - The index of the field
    /// * `s` - The string to parse from
    fn parse_field(&mut self, index: usize, s: &str) -> Result<(), anyhow::Error>;
}

/// Creates a map from the struct field indices to the CSV column indices.
///
/// # Arguments
/// * `header` - The CSV header fields.
fn create_index_map<Record: StructInfo>(header: &[String]) -> Result<Vec<usize>> {
    let mut m = vec![0usize; Record::NUM_FIELDS];

    for (i, column_index) in m.iter_mut().enumerate() {
        let name = Record::get_field_name(i);
        match header.iter().position(|c| *c == name) {
            Some(index) => {
                *column_index = index;
            }
            None => {
                bail!("Cannot find field {}", name);
            }
        }
    }

    Ok(m)
}

pub fn deserialize_csv<Record, R>(rdr: R) -> Result<Vec<Record>>
where
    R: Read,
    Record: StructInfo,
{
    // create parser, read the header record and build struct fields -> CSV column map
    let mut parser = CSVParser::new(rdr);
    let header = parser.read_header_record()?;
    let field_map = create_index_map::<Record>(&header)?;

    // parse the records...
    let mut record_raw = vec![String::new(); header.len()];
    let mut records = Vec::new();
    loop {
        match parser.read_record(&mut record_raw)? {
            ReaderResult::Data(_) => {
                let mut record: Record = Default::default();

                for (field_index, record_index) in field_map.iter().enumerate() {
                    let s = record_raw[*record_index].as_str();
                    record.parse_field(field_index, s)?;
                }

                records.push(record);
            }
            ReaderResult::Eof => {
                break;
            }
        }
    }

    Ok(records)
}
