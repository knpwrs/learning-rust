extern crate minigrep;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    // The result is the unit value, so there is no need to "unwrap"
    if let Err(e) = minigrep::run(config) {
        eprintln!("Error: {:?}", e);
        process::exit(1);
    }
}
