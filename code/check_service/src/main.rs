    // Rust 1.19, Hyper 0.11, tokio-core 0.1, futures 0.1

    extern crate reqwest;

    fn main() {
        let body = reqwest::get("https://www.rust-lang.org").unwrap().text().unwrap();
        println!("body = {:?}", body);
    }
