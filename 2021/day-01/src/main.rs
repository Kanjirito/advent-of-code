use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const INPUT: &str = "input";

fn main() {
    let numbers = load_data(INPUT);
    println!("{}", count_increases(&numbers));
    println!("{}", three_measurement_comparison(&numbers))
}

/// Loads the input file and returns a Vec with the numbers
fn load_data(path: &str) -> Vec<u16> {
    let file = File::open(path).expect("No input file found");
    let reader = BufReader::new(file);
    let mut numbers: Vec<u16> = Vec::new();
    for line in reader.lines() {
        match line.expect("Wrong line").parse() {
            Ok(num) => numbers.push(num),
            Err(_) => continue,
        };
    }
    numbers
}

/// Counts the number of time the numbers increase
fn count_increases(numbers: &[u16]) -> u16 {
    let mut counter: u16 = 0;
    let mut prev_number: u16 = numbers[0];
    for number in &numbers[1..] {
        if number > &prev_number {
            counter += 1;
        }
        prev_number = *number;
    }
    counter
}

fn three_measurement_comparison(numbers: &[u16]) -> u16 {
    let mut counter: u16 = 0;
    let mut last_sum = numbers[0] + numbers[1] + numbers[2];
    for (i, thrid_number) in numbers.iter().enumerate() {
        if i < 3 {
            continue;
        }
        let current_sum = numbers[i - 2] + numbers[i - 1] + thrid_number;
        if current_sum > last_sum {
            counter += 1;
        }
        last_sum = current_sum;
    }

    counter
}
