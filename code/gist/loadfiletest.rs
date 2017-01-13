/// used mods
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::error::Error;

pub fn newreader(filename: String) -> BufReader<File> {
    let path = Path::new(&filename);
    let f = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
        Ok(file) => file,
    };
    BufReader::new(f)
}

pub fn getline(mut f: &mut BufReader<File>) -> Option<String> {
    //         ^ important
    let mut line = String::new();
    if f.read_line(&mut line).unwrap() == 0 {
        None
    } else {
        Some(line)
    }
}

pub fn getline2(mut f: &mut BufReader<File>) -> Option<String> {
    getline(&mut f)
}

fn main() {
    let mut f = newreader("loadfiletest.rs".to_string());
    let line1: String = getline(&mut f).unwrap();
    println!("{}", line1);
    let line2: String = getline2(&mut f).unwrap();
    println!("{}", line2);
}
