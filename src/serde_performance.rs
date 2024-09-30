use std::{error::Error, io};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record <'a> {
    country: &'a str,
    city: &'a str,
    accent_city: &'a str,
    region: &'a str,
    population: Option<u64>,
    latitude: f64,
    longitude: f64
}

pub(crate) fn run() -> Result<u64, Box<dyn Error>> {
    let mut reader: csv::Reader<io::Stdin> = csv::Reader::from_reader(io::stdin());
    let mut raw_record: csv::StringRecord = csv::StringRecord::new();
    let headers: csv::StringRecord = reader.headers()?.clone();
    let mut count: u64 = 0;

    while reader.read_record(&mut raw_record)? {
        let record: Record = raw_record.deserialize(Some(&headers))?;
        if record.country == "us" && record.region == "MA" {
            count += 1;
        }
    }
    Ok(count)
}