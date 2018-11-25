// #![feature(proc_macro)]
#[macro_use]

// extern
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

// mods
mod aspects;
mod common;

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
    println! {"{:?} \n", elementlist};
    // access submap
    let ehash = String::from("elements");
    let evalue = elementlist.get(&ehash).unwrap();
    println! {"{:?} \n", evalue};
    // print first value of Vector
    println! {"{:?} \n", evalue[0]};
    // ## Sequence stop

    // second version by HashMap
    let elementlist = aspects::elementreading::read_elementlist_file_by_hashmap();
    println! {"{:?} \n", elementlist};

    // third version, automatic import of full file, by using testout1.json
    println!("Elementlist in one reading \n",);
    let e = aspects::elementreading::read_elementlist_file_at_once();
    println!("{:?}", e);
    println!("works, but not with the original file, because of null values and empty f64 values",);

    // fourth version, does it get better with using deserialize_with?
    // yes it works
    // TODO convert number from string to u16
    let e = aspects::elementreading::read_elementlist_file_and_resolve_nulls();
    println!("{:?}\n\n\n", e);
    println!("{:?}\n", e[5]);
}
