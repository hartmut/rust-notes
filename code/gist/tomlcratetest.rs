// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
/// for initalization and configuration

/// used mods
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::path::Path;
use std::collections::BTreeMap;
use toml::{Value, Parser};

/// configuration
#[derive(Debug, Clone)]
pub struct Configuration {
    tick: u64,
    o2_per_person: u64,
    filename_player: String,
    filename_station: String,
    filename_module: String,
    filename_elements: String, // periodic table
    filename_components: String,
}

impl Configuration {
    pub fn load_config(args: Vec<String>) -> Configuration {

        // configuration is here server/src/data/config.toml
        let path = Path::new(&args[1]);
        let display = path.display();
        let mut input = String::new();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
            Err(why) => panic!("couldn't open {}: {}", display, why.description()),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        match file.read_to_string(&mut input) {
            Err(why) => panic!("couldn't read {}: {}", display, why.description()),
            // Ok(_) => print!("{} contains:\n{}\n\n", display, input),
            Ok(_) => print!(""),
        }

        let mut parser = Parser::new(&input);

        let toml = match parser.parse() {
            None => {
                for err in &parser.errors {
                    let (loline, locol) = parser.to_linecol(err.lo);
                    let (hiline, hicol) = parser.to_linecol(err.hi);
                    println!("{}:{}:{}-{}:{} error: {}",
                             display,
                             loline,
                             locol,
                             hiline,
                             hicol,
                             err.desc)
                }
                panic!("configuration error");
            }
            Some(toml) => toml,
        };

        // decompose the toml file.
        // IDEA: try to interpret the toml file in a less complex way
        let conf: BTreeMap<String, Value> = toml;
        let global = conf.get(&"global".to_string()).unwrap();
        let player = conf.get(&"playerdata".to_string()).unwrap();
        let station = conf.get(&"structuredata".to_string()).unwrap();
        let module = conf.get(&"moduledata".to_string()).unwrap();
        let elements = conf.get(&"elements".to_string()).unwrap();
        let components = conf.get(&"componentdata".to_string()).unwrap();

        // create the Configuration structure
        Configuration {
            tick: global.lookup("tick").unwrap().as_integer().unwrap() as u64,
            o2_per_person: global.lookup("02player").unwrap().as_integer().unwrap() as u64,
            filename_player: player.lookup("datafile").unwrap().as_str().unwrap().to_string(),
            filename_station: station.lookup("datafile_station")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
            filename_module: module.lookup("datafile").unwrap().as_str().unwrap().to_string(),
            filename_elements: elements.lookup("datafile").unwrap().as_str().unwrap().to_string(),
            filename_components: components.lookup("datafile")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
        }
    }
}
