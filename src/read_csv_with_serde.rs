use std::error::Error;
use std::fs::File;

use csv::Reader;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    latitude: f64,
    longitude: f64,
    #[serde(deserialize_with = "csv::invalid_option")]
    population: Option<u64>,
    city: String,
    state: String,
}

pub(crate) fn run(file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader: Reader<File> = Reader::from_path(file_path)?;
    for result in reader.deserialize() {
        let record: Record = result?;
        println!("{:#?}", record);
    }
    Ok(())
}

pub(crate) fn run_with_debug(file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader: Reader<File> = Reader::from_path(file_path)?;
    for result in reader.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
