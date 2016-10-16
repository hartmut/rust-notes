extern crate rand;
extern crate futures;
extern crate tokio_core;

use rand::{Rng, thread_rng};
use futures::stream::Stream;
use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;
use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    // initialize

    // read configuration
    println!("Reading recipes");
    let recipes = read_recipes("data/whattoeat.txt");

    // initialize listener parameters
    println!("Starting recipes server...");
    let mut lp = Core::new().unwrap();
    // let handle = lp.handle();
    let addr = "127.0.0.1:8899".parse().unwrap();

    // Create new TCP listener
    let listener = match TcpListener::bind(&addr, &lp.handle()) {
        Ok(l) => l,
        Err(e) => {
            println!("something went wrong: {}", e);
            std::process::exit(1);
        }
    };

    // create server
    let clients = listener.incoming();
    let answer = clients.and_then(|(socket, _peer_addr)| {
        println!("got connection from {}", _peer_addr);
        let onerecipe = thread_rng().choose(&recipes).unwrap();
        tokio_core::io::write_all(socket, onerecipe.as_bytes())
    });

    let server = answer.for_each(|(_socket, _welcome)| {
        Ok(())
    });

    // and let it run
    lp.run(server).unwrap();
}

fn read_recipes(file: &str) -> Vec<String> {
    let mut recipes = Vec::new();
    let recipe_file = File::open(file).unwrap();
    let mut recipe = String::new();

//TODO rewrite for line structure
    for readline in BufReader::new(recipe_file).lines() {
        if let Ok(line) = readline {
            if line == "%" {
                recipes.push(recipe.clone());
                recipe.clear();
            }else {
                recipe.push_str(&line);
                recipe.push('\n');
            }
        }
    }
    recipes.push(recipe);
    recipes
}
