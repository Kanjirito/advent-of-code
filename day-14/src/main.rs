use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let (mut polymer, rules) = load_input();
    for _ in 0..10 {
        let mut new_polymer: Vec<char> = Vec::new();
        for index in 0..polymer.len() - 1 {
            let pair = format!("{}{}", polymer[index], polymer[index + 1]);
            new_polymer.push(polymer[index]);
            if let Some(c) = rules.get(&pair) {
                new_polymer.push(*c);
            }
        }
        new_polymer.push(polymer[polymer.len() - 1]);
        polymer = new_polymer;
    }
    let mut counts: Vec<usize> = count_chars(&polymer).values().copied().collect();
    counts.sort_unstable();
    println!(
        "Solution for part 1: {}",
        counts[counts.len() - 1] - counts[0]
    );
}

fn count_chars(chars: &[char]) -> HashMap<char, usize> {
    let mut counter_map: HashMap<char, usize> = HashMap::new();
    for c in chars {
        let counter = counter_map.entry(*c).or_insert(0);
        *counter += 1;
    }
    counter_map
}

fn load_input() -> (Vec<char>, HashMap<String, char>) {
    let file = File::open("input").expect("No input file found");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let start = lines.next().unwrap().unwrap().chars().collect();
    let mut rules: HashMap<String, char> = HashMap::new();
    lines.next();
    for line in lines {
        let text = line.unwrap();
        let mut split = text.split(" -> ");
        rules.insert(
            split.next().unwrap().to_string(),
            split.next().unwrap().parse().unwrap(),
        );
    }
    (start, rules)
}
