mod write_csv;

use std::process;

fn main() {
    if let Err(err) = write_csv::run() {
        println!("{}", err);
        process::exit(1);
    }
}
