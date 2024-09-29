use csv::{WriterBuilder};
use std::{env, error::Error, ffi::OsString};
use std::fs::File;

pub(crate) fn run() -> Result<(), Box<dyn Error>> {
    let file_path: OsString = get_first_arg()?;
    let file : File = File::create(file_path)?;
    let mut writer = WriterBuilder::new()
        .delimiter(b'\t')
        .quote_style(csv::QuoteStyle::NonNumeric)
        .from_writer(file);
    
    writer.write_record(["City", "State", "Population", "Latitude", "Longitude"])?;
    writer.write_record(["Davidson Landing", "AK", "", "65.241944", "-165.2716667"])?;
    writer.write_record(["Kenai", "AK", "7610", "60.5544444", "-151.2583333"])?;
    writer.write_record(["Oakman", "AL", "", "33.7133333", "-87.38861111"])?;
    writer.write_record(vec!["Newyork", "NY", "", "", ""])?;
    writer.write_record(&csv::StringRecord::from(vec![
        "Los Angeles",
        "CA",
        "",
        "",
        "",
    ]))?;
    // Flush the buffer
    writer.flush()?;

    Ok(())
}


fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("Expected 1 argument but got none")),
        Some(filepath) => Ok(filepath),
    }
}
