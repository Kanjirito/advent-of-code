use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input();
}

fn load_input() -> {
    let file = File::open("input").expect("No input file found");
    let reader = BufReader::new(file);
}
