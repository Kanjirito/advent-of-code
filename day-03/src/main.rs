use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let priority: HashMap<char, usize> = HashMap::from_iter(
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .enumerate()
            .map(|(i, c)| (c, i + 1)),
    );
    let input = load_input();
    println!("Sum of priorities: {}", part_1(&input, &priority));
    println!("Sum of second priorities {}", part_2(&input, &priority));
}

fn part_1(input: &[Rucksack], priority: &HashMap<char, usize>) -> usize {
    let mut result: usize = 0;
    for rucksack in input {
        for element in &rucksack.first_compartment {
            if rucksack.second_compartment.contains(element) {
                result += priority[element]
            }
        }
    }
    result
}

fn part_2(input: &[Rucksack], priority: &HashMap<char, usize>) -> usize {
    let mut result: usize = 0;
    for chunk in input.chunks(3) {
        for element in &chunk[0].all {
            if chunk[1].all.contains(element) && chunk[2].all.contains(element) {
                result += priority[element];
            }
        }
    }
    result
}

fn load_input() -> std::vec::Vec<Rucksack> {
    let file = File::open("input").expect("No input file found");
    let reader = BufReader::new(file);
    let mut result: Vec<Rucksack> = Vec::new();
    for line in reader.lines().map(|l| l.expect("Failed to read file")) {
        result.push(Rucksack::new(line));
    }
    result
}

#[derive(Debug)]
struct Rucksack {
    first_compartment: HashSet<char>,
    second_compartment: HashSet<char>,
    all: HashSet<char>,
}

impl Rucksack {
    fn new(string: String) -> Self {
        let mid = string.len() / 2;
        let mut first_compartment = HashSet::with_capacity(mid);
        let mut second_compartment = HashSet::with_capacity(mid);
        let mut all = HashSet::with_capacity(string.len());
        for (i, c) in string.chars().enumerate() {
            if i < mid {
                first_compartment.insert(c);
            } else {
                second_compartment.insert(c);
            }
            all.insert(c);
        }
        Self {
            first_compartment,
            second_compartment,
            all,
        }
    }
}
