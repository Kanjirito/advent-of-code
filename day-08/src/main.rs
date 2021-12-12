use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input();

    // Part 1
    let mut counter: usize = 0;
    // Because who needs readable loops?
    for code in input.iter().flat_map(|x| &x.1).map(|x| x.len()) {
        match code {
            2 | 3 | 4 | 7 => counter += 1,
            _ => {}
        }
    }
    println!("Solution for part 1: {}", counter);
}

fn load_input() -> Vec<(Vec<String>, Vec<String>)> {
    let file = File::open("input").expect("No input file found");
    let reader = BufReader::new(file);
    let mut codes: Vec<(Vec<String>, Vec<String>)> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split(" | ");
        let first: Vec<String> = split
            .next()
            .unwrap()
            .split(' ')
            .map(|x| x.to_string())
            .collect();
        let second: Vec<String> = split
            .next()
            .unwrap()
            .split(' ')
            .map(|x| x.to_string())
            .collect();
        codes.push((first, second));
    }
    codes
}
