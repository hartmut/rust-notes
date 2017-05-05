
// #![feature(proc_macro)]
#[macro_use]

// extern
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

// mods
mod common;

// uses
use serde_json::{Value, Error};
// use serde::de;
use common::fileoperations::*;

// all the elemenets
// updates from https://raw.githubusercontent.com/Bowserinator/Periodic-Table-JSON/master/PeriodicTableJSON.json

// TODO design struct and read from file #NEXT
// IDEA move this to the structure module? do I need this data anywhere else?

#[derive(Serialize, Deserialize, Debug)]
struct Element {
    name: String,
    appearance: String,
    atomic_mass: f64,
    boil: f64, // in Kelvin
    category: String,
    color: String,
    density: f64,
    discovered_by: String,
    melt: f64, // in Kelvin
    molar_heat: f64, // in Kelvin
    named_by: String,
    number: u32, // atomic number
    period: u32,
    phase: String,
    source: String,
    spectral_img: String,
    summary: String,
    symbol: String,
    xpos: u32,
    ypos: u32,
}

type ElementListVec = Vec<Element>;

// read Elementlist from file
fn read_elementlist_file() -> serde_json::Value {
    // TODO send filename

    let result = read_file_to_string("PeriodicTableJSON.json".to_string());

    // ElementList
    let e: Value = serde_json::from_str(&result).unwrap();
    // let elementlist: ElementListVec = <std::vec::Vec<recipes::elements::Element> as Trait>::serde_json::from_str(&result).unwrap();
    // let elementlist = match checker_elementlist {
    //     Ok(elementlist) => elementlist,
    //     Err(error) => {
    //         panic!("somethings is wrong with the deserelization of the elementsfile: {:?}",
    //                error)
    //     }
    // };
    // elementlist
    e
}

fn main() {
    let elementlist = read_elementlist_file();
    println!{"{} \n", elementlist};
    // println!{"source: {}", elementlist["elements"]};

    let v = vec![1, 2, 3, 4];
    println!("{:?}", v);
}
