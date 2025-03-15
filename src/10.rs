use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::create("example.txt").unwrap();
    write!(file, "Hello, world!").unwrap();
}
