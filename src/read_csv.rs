use std::{env, error::Error, ffi::OsString, fs::File};

use csv::{Reader, StringRecord};

pub(crate) fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let mut reader: Reader<File> = Reader::from_path(file_path)?;
    for result in reader.records() {
        let record: StringRecord = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("Expected 1 argument, but got none.")),
        Some(file_path) => Ok(file_path),
    }
}
