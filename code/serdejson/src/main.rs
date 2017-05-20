// #![feature(proc_macro)]
#[macro_use]

// extern
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

// mods
mod common;
mod aspects;

// uses
use std::collections::HashMap;



// all the elemenets
// updates from https://raw.githubusercontent.com/Bowserinator/Periodic-Table-JSON/master/PeriodicTableJSON.json


fn main() {
    // first version
    let elementlist = aspects::elementreading::read_elementlist_file();
    // println!{"{:?} \n", elementlist};
    // access submap
    // let ehash = String::from("elements");
    // let evalue = elementlist.get(&ehash).unwrap();
    // println!{"{:?} \n", evalue};
    // print first value of Vector
    // println!{"{:?} \n", evalue[0]};

    // second version
    // TODO hier weiter
    let e: aspects::elementreading::Element = aspects::elementreading::read_elementlist_file2();
    // println!{"{:?} \n", e};

    // Vector
    // let v = vec![1, 2, 3, 4];
    // println!("{:?}", v);

    // Hasmaps
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);
    // println!("{:?}", scores);

    aspects::elementreading::create_example();

}
