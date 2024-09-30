mod serde_performance;

use std::process;

fn main() {
    match serde_performance::run() {
        Ok(count) => {
            println!("{}", count);
        }
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    }
}
