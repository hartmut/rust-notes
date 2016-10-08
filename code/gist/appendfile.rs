use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
    let mut file =
        OpenOptions::new()
        .write(true)
        .append(true)
        .open("my-file")
        .unwrap();

    if let Err(e) = writeln!(file, "A new line!") {
        println!("{}", e);
    }
}
