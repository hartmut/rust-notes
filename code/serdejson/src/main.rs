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

    // create easy examplefile, how does it look?
    aspects::elementreading::create_example();

    // ## Sequence start
    // first version naive interpretation
    let elementlist = aspects::elementreading::read_elementlist_file();
    println!{"{:?} \n", elementlist};
    // access submap
    let ehash = String::from("elements");
    let evalue = elementlist.get(&ehash).unwrap();
    println!{"{:?} \n", evalue};
    // print first value of Vector
    println!{"{:?} \n", evalue[0]};
    // ## Sequence stop

    // second version by HashMap
    // let e: aspects::elementreading::Element =
    aspects::elementreading::read_elementlist_file_by_hashmap();
    // println!{"{:?} \n", e};

    // TODO hier weiter
    // third version, does it get better with visitors?
    aspects::elementreading::read_elementlist_file_by_visiting();

}
