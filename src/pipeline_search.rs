use std::{env, error::Error, io};

use csv::{ByteRecord, Reader,  Writer};

pub(crate) fn run() -> Result<(), Box<dyn Error>> {
    // Get query from the positional arguments
    let query: String = match env::args().nth(1) {
        None => return Err(From::from("expected 1 argument, but got none")),
        Some(query) => query,
    };

    let mut reader: Reader<io::Stdin> = Reader::from_reader(io::stdin());
    let mut writer: Writer<io::Stdout> = Writer::from_writer(io::stdout());

    writer.write_record(reader.headers()?)?;

    for result in reader.byte_records() {
        let record: ByteRecord = result?;
        if record.iter().any(|field| field == query.as_bytes()) {
            writer.write_record(&record)?;
        }
    }

    writer.flush()?;

    Ok(())
}
