use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let input = load_input();
    println!("Part 1: {}", solve(&input, 4));
    println!("Part 2: {}", solve(&input, 14));
}

fn solve(string: &String, count: usize) -> usize {
    for (i, slice) in string.as_bytes().windows(count).enumerate() {
        if HashSet::<&u8>::from_iter(slice).len() == count {
            return i + count;
        }
    }
    unreachable!()
}

fn load_input() -> std::string::String {
    let mut file = File::open("input").expect("No input file found");
    let mut result = String::new();
    file.read_to_string(&mut result).unwrap();
    result.trim().into()
}
