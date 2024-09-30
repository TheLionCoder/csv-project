use csv::{ByteRecord, Reader};
use std::{error::Error, io};

pub(crate) fn run() -> Result<u64, Box<dyn Error>> {
    let mut reader: Reader<io::Stdin> = Reader::from_reader(io::stdin());
    let mut record: ByteRecord = ByteRecord::new();

    let mut count: u64 = 0;

    while reader.read_byte_record(&mut record)? {
        if &record[0] == b"us" && &record[3] == b"MA" {
            count += 1;
        }
    }

    Ok(count)
}
