// extern crate tokio_core;
extern crate rand;
extern crate futures;
extern crate tokio_core;
extern crate tokio_timer;
extern crate tokio_service;
// extern crate tokio_proto as proto;
// extern crate tokio_line as line;
// extern crate tokio_middleware as middleware;

// use tokio_core::reactor;
// use tokio_core::reactor::Tick;
// use tokio_core::tcp::TcpListener;
// use futures::Future;
use rand::{Rng, thread_rng};
use futures::stream::Stream;
use tokio_core::reactor::Core;
use tokio_core::reactor;
use tokio_core::net::TcpListener;
use tokio_timer::Timer;
// use service::Service;
use std::time::Duration;
use std::io::{self, Write, BufReader, BufRead};
use std::fs::File;

struct Listener {
    // socket: TcpListener,
    quotes: Vec<String>
}

impl Listener {
    fn tick(&mut self) -> io::Result<Tick> {
        while let Some(mut conn) = try!(self.socket.accept()) {
            let quote = thread_rng().choose(&self.quotes).unwrap();
            try!(conn.write_all(quote.as_bytes()));
        }

        Ok(Tick::WouldBlock)
    }
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

// https://github.com/tokio-rs/tokio-proto/blob/0bfe489f2bd97307024df8f0c7a7ed112f8253e5/examples/listen.rs

fn main() {
    // read configuration
    println!("Reading recipes");
    let recipes = read_recipes("data/whattoeat.txt");

    // initialize listener parameters
    println!("Starting recipes server...");
    let mut lp = Core::new().unwrap();
    let handle = lp.handle();
    let addr = "127.0.0.1:8899".parse().unwrap();

    // Create the new TCP listener
    let listener = TcpListener::bind(&addr, &lp.handle()).unwrap();

    // let reactor = reactor::default().unwrap();

//     reactor.handle().oneshot(|| {
//         let listener = match TcpListener::bind(&addr) {
//             Ok(l) => l,
//             Err(e) => {
//                 println!("Error creating listener: {}", e);
//                 std::process::exit(1);
//             }
//         };
//
//         reactor::schedule(Listener {socket: listener, quotes: quotes});
//
//         Ok(())
//     });
//     reactor.run();

    let clients = listener.incoming().map(move |(socket, addr)| {
        (Listener {
            quotes: recipes,
        }.tick)
    });

    let handle = lp.handle();

    let server = clients.for_each(|(client, addr)| {
        handle.spawn(client.then(move |res| {
            match res {
                Ok((a, b)) => {
                    println!("proxied {}/{} bytes for {}", a, b, addr)
                }
                Err(e) => println!("error for {}: {}", addr, e),
            }
            futures::finished(())
        }));
        Ok(())
    });

    lp.run(server).unwrap();

}
