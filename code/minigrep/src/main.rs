use std::env;
use std::process;

use minigrep;
use minigrep::Config;

// rust learning
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // println!("Searching for {:?}", config.query);
    // println!("in file {:?}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error {}", e);

        process::exit(1);
    };
}
