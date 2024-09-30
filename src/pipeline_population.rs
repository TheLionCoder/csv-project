use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};
use std::io::{Stdin, Stdout};
use std::{env, error::Error, io};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    city: String,
    state: String,
    population: Option<u64>,
    latitude: f64,
    longitude: f64,
}

pub(crate) fn run() -> Result<(), Box<dyn Error>> {
    let minimun_pop: u64 = match env::args().nth(1) {
        None => return Err(From::from("Please specify a minimum population")),
        Some(arg) => arg.parse()?,
    };

    let mut reader: Reader<Stdin> = Reader::from_reader(io::stdin());
    let mut writer: Writer<Stdout> = Writer::from_writer(io::stdout());

    for result in reader.deserialize() {
        let record: Record = result?;
        if record.population.map_or(false, |pop| pop >= minimun_pop) {
            writer.serialize(&record)?;
        }
    }

    writer.flush()?;

    Ok(())
}
