use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let mut input = load_input();
    input.sort_unstable();
    let median = input[input.len() / 2];
    let mut fuel_counter: usize = 0;
    for crab in &input {
        fuel_counter += {
            if crab >= &median {
                crab - median
            } else {
                median - crab
            }
        }
    }
    println!("Solution for part 1: {}", fuel_counter);
    fuel_counter = 0;
    let average: usize = input.iter().sum::<usize>() / input.len();
    for crab in &input {
        let steps: usize;
        if crab >= &average {
            steps = crab - average;
        } else {
            steps = average - crab
        }
        fuel_counter += steps * (steps + 1) / 2;
    }
    println!("Solution for part 2: {}", fuel_counter);
}


fn load_input() -> Vec<usize> {
    let file = File::open("input").expect("No input file found");
    let reader = BufReader::new(file);
    reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}
