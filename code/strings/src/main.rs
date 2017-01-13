// macros and plugins
#![allow(dead_code)]
#![feature(proc_macro)]
#[macro_use]

mod fileoperations;
mod tests;

use fileoperations::*;



fn main() {
    println!("\nTest of Strings");

    // BufReader Test
    let mut f = newreader("src/tests/testdata/testfile".to_string());

}
