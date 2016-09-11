//necessary module
//To clarify what @mcarton said, Args implements the ExactSizeIterator, so there's no need to collect the results into a vector. –  Timidger yesterday

use std::env;

fn main() {

    // move all arguments to a string vector
    let args: Vec<String> = env::args().collect();

    // panic if vector is too short
    if args.len() <= 1 {
        panic!("I need your input");
    }
}
