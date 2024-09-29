use csv::{Reader, ReaderBuilder};
use std::error::Error;
use std::io::Stdin;

pub(crate) fn run() -> Result<(), Box<dyn Error>> {
    let mut reader: Reader<Stdin> = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(std::io::stdin());

    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
