use std::error::Error;
use std::io;
use std::io::Stdout;

use csv::Writer;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Record<'a> {
    city: &'a str,
    state: &'a str,
    population: Option<u64>,
    latitude: f64,
    longitude: f64,
}

pub(crate) fn run() -> Result<(), Box<dyn Error>> {
    let mut writer: Writer<Stdout> = Writer::from_writer(io::stdout());

    writer.serialize(Record {
        city: "Davidson Landing",
        state: "AK",
        population: None,
        latitude: 65.2419444,
        longitude: -165.2716667,
    })?;

    writer.serialize(Record {
        city: "Kenai",
        state: "AK",
        population: Some(7610),
        latitude: 60.5544444,
        longitude: -151.2583333,
    })?;

    writer.serialize(Record {
        city: "Oakman",
        state: "AL",
        population: None,
        latitude: 33.7133333,
        longitude: -87.3886111,
    })?;

    writer.flush()?;
    Ok(())
}
