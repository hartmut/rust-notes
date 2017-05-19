// uses
use serde_json::{Value, Error};
use serde_json;
use common::fileoperations::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Element {
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
    number: String,
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

    let result = read_file_to_string("src/testout1.json".to_string());

    // ElementList
    let e: ElementListVec = serde_json::from_str(&result).unwrap();
    // let ehash = String::from("elements");
    // let evalue = e.get(&ehash).unwrap();
    // evalue
    e
}

pub fn create_example() {

    let e: Element = Element {
        name: "Helium".to_string(),
        appearance: "colorless gas, exhibiting a red-orange glow when placed in a high-voltage \
                     electric field"
            .to_string(),
        atomic_mass: 4.0026022,
        boil: 4.222,
        category: "noble gas".to_string(),
        color: "".to_string(),
        density: 0.1786,
        discovered_by: "Pierre Janssen".to_string(),
        melt: 0.95,
        molar_heat: 0.0,
        named_by: "".to_string(),
        number: "2".to_string(),
        period: 1,
        phase: "Gas".to_string(),
        source: "https://en.wikipedia.org/wiki/Helium".to_string(),
        spectral_img: "https://en.wikipedia.org/wiki/File:Helium_spectrum.jpg".to_string(),
        summary: "Helium is a chemical element with symbol He and atomic number 2. It is a \
                  colorless, odorless, tasteless, non-toxic, inert, monatomic gas that heads the \
                  noble gas group in the periodic table. Its boiling and melting points are the \
                  lowest among all the elements."
            .to_string(),
        symbol: "He".to_string(),
        xpos: 18,
        ypos: 1,
    };

    let f: String = serde_json::to_string(&e).unwrap();
    let b0: u64 = write_string_to_file("src/testout.json".to_string(), &f);

    let v = vec![e];
    let g: String = serde_json::to_string(&v).unwrap();
    let b1: u64 = write_string_to_file("src/testout1.json".to_string(), &g);
}
