use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let (first_char, mut polymer_pairs, rules) = load_input();
    for loop_n in 0..40 {
        let mut new_polymer_pairs: HashMap<String, usize> = HashMap::new();
        for (key, value) in polymer_pairs {
            for new_pair in rules[&key].iter() {
                let counter = new_polymer_pairs.entry(new_pair.to_string()).or_insert(0);
                *counter += value;
            }
        }
        polymer_pairs = new_polymer_pairs;
        if loop_n == 9 {
            let part_1_char_count = count_chars(&polymer_pairs, first_char);
            let mut part_1_counts: Vec<usize> = part_1_char_count.values().copied().collect();
            part_1_counts.sort_unstable();
            println!(
                "Solution for part 1: {}",
                (part_1_counts[part_1_counts.len() - 1] - part_1_counts[0])
            );
        }
    }
    let part_2_char_count = count_chars(&polymer_pairs, first_char);
    let mut part_2_counts: Vec<usize> = part_2_char_count.values().copied().collect();
    part_2_counts.sort_unstable();
    println!(
        "Solution for part 2: {}",
        (part_2_counts[part_2_counts.len() - 1] - part_2_counts[0])
    );
}

fn count_chars(polymer_pairs: &HashMap<String, usize>, first_char: char) -> HashMap<char, usize> {
    let mut counter_map: HashMap<char, usize> = HashMap::new();
    for (key, value) in polymer_pairs {
        let counter = counter_map.entry(key.chars().nth(1).unwrap()).or_insert(0);
        *counter += value;
    }
    // You need to manually add the first character because it would be skipped since you only count the second element of pairs
    let counter = counter_map.entry(first_char).or_insert(0);
    *counter += 1;
    counter_map
}

fn load_input() -> (char, HashMap<String, usize>, HashMap<String, [String; 2]>) {
    let file = File::open("input").expect("No input file found");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let start: Vec<char> = lines.next().unwrap().unwrap().chars().collect();
    let mut polymer: HashMap<String, usize> = HashMap::new();
    for i in 0..start.len() - 1 {
        let pair = format!("{}{}", start[i], start[i + 1]);
        let counter = polymer.entry(pair).or_insert(0);
        *counter += 1;
    }

    let mut rules: HashMap<String, [String; 2]> = HashMap::new();
    lines.next();
    for line in lines {
        let text = line.unwrap();
        let mut split = text.split(" -> ");
        let first: Vec<char> = split.next().unwrap().chars().collect();
        let second: char = split.next().unwrap().parse().unwrap();
        rules.insert(
            first.iter().copied().collect::<String>(),
            [
                format!("{}{}", first[0], second),
                format!("{}{}", second, first[1]),
            ],
        );
    }
    (start[0], polymer, rules)
}
