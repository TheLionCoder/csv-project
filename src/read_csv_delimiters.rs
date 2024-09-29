use csv::{Reader, ReaderBuilder};
use std::error::Error;
use std::fs::File;

pub(crate) fn run(file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader: Reader<File> = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .double_quote(false)
        .escape(Some(b'\\'))
        .flexible(true)
        .comment(Some(b'#'))
        .from_path(file_path)?;

    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
