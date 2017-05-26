// uses
use serde_json::{Value, Error};
use serde_json;
use common::fileoperations::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionElement {
    name: String,
    appearance: String,
    atomic_mass: f64,
    boil: f64,
    category: String,
    #[serde(default)]
    color: Option<String>,
    density: f64,
    discovered_by: String,
    melt: f64,
    #[serde(default)]
    molar_heat: Option<f64>,
    named_by: Option<String>,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Element {
    name: String,
    appearance: String,
    atomic_mass: f64,
    boil: f64,
    category: String,
    #[serde(default)]
    color: String,
    density: f64,
    discovered_by: String,
    melt: f64,
    #[serde(default)]
    molar_heat: f64,
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

type OptionElementListVec = Vec<OptionElement>;
type ElementListVec = Vec<Element>;

// read Elementlist from file
pub fn read_elementlist_file() -> serde_json::Value {

    let result = read_file_to_string("src/PeriodicTableJSON.json".to_string());

    // ElementList
    let e: Value = serde_json::from_str(&result).unwrap();
    // let elementlist: OptionElementListVec = <std::vec::Vec<recipes::elements::OptionElement> as Trait>::serde_json::from_str(&result).unwrap();
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

pub fn read_elementlist_file_by_hashmap() {

    println!("at first we get the file as a string", );
    let mut result = read_file_to_string("src/PeriodicTableJSON.json".to_string());
    // ElementList
    println!{"{:?} \n", &result};

    println!("This should be an array", );
    let mut e: Value = serde_json::from_str(&result).unwrap();
    let mut ehash = String::from("elements");
    let earray = e.get(&ehash).unwrap();
    println!{"{:?} \n", earray};

    println!("and now we geht the first element which is a HashMap", );
    println!("{:?}\n", earray[0]);

    println!("lets take a look at the appearance of the first element of the array", );
    ehash = String::from("appearance");
    let appearance: String = earray[0].get(&ehash).unwrap().to_string();
    println!("{:?}", appearance);
    ehash = String::from("atomic_mass");
    let atomic_mass = earray[0].get(&ehash).unwrap().to_string();
    println!("{:?}", atomic_mass);
}

// fn Element_Builder_from_OptElement(input: OptionElement) -> Element {
//
// }

pub fn read_elementlist_file_at_once() -> ElementListVec {

    let result = read_file_to_string("src/testout2.json".to_string());
    let e: Result<OptionElementListVec, Error> = serde_json::from_str(&result);

    let OptElementList = match e {
        Ok(elementlist) => elementlist,
        Err(error) => {
            panic!("somethings is wrong with the deserialization of the elementsfile: {:?}",
                   error);
        }
    };

    // convert the structure with options to the useable structure

    let mut iterator = OptElementList.iter();

    loop {
        match iterator.next() {
            Some(x) => {
                println!("{:?}", x);
            }
            None => break,
        }
    }

    Vec::new()

}




pub fn read_elementlist_file_by_visiting() {
    println!("just a stub now", );
}

pub fn create_example() {

    let e: OptionElement = OptionElement {
        name: "Helium".to_string(),
        appearance: "colorless gas, exhibiting a red-orange glow when placed in a high-voltage \
                     electric field"
            .to_string(),
        atomic_mass: 4.0026022,
        boil: 4.222,
        category: "noble gas".to_string(),
        color: Some("".to_string()),
        density: 0.1786,
        discovered_by: "Pierre Janssen".to_string(),
        melt: 0.95,
        molar_heat: Some(0.0),
        named_by: Some("".to_string()),
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

    // write one element filename
    let f: String = serde_json::to_string(&e.clone()).unwrap();
    let b0: u64 = write_string_to_file("src/testout.json".to_string(), &f);

    // write two element file, TODO
    let mut v: OptionElementListVec = vec![e.clone()];
    v.push(e.clone());
    let g: String = serde_json::to_string(&v).unwrap();
    let b1: u64 = write_string_to_file("src/testout1.json".to_string(), &g);
}
