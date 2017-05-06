// uses
use serde_json::{Value, Error};
use serde_json;

// use serde::de;
use common::fileoperations::*;

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
pub fn read_elementlist_file() -> serde_json::Value {

    let result = read_file_to_string("src/PeriodicTableJSON.json".to_string());

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

pub fn read_elementlist_file2() -> ElementListVec {

    let result = read_file_to_string("src/PeriodicTableJSON.json".to_string());

    // ElementList
    let e: Value = serde_json::from_str(&result).unwrap();
    let ehash = String::from("elements");
    let evalue = e.get(&ehash).unwrap();
    evalue

}
