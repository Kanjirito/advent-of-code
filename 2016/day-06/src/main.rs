use std::fs::File;
use std::io::BufReader;

use utils::{BufReadExt, Counter};

fn main() {
    let input = load_input("input");
    let [r1, r2] = solve(&input);
    println!("Solution for part 1: {r1}");
    println!("Solution for part 2: {r2}");
}

fn solve(lines: &[String]) -> [String; 2] {
    let mut counters: [Counter<char>; 8] = [
        Counter::new(),
        Counter::new(),
        Counter::new(),
        Counter::new(),
        Counter::new(),
        Counter::new(),
        Counter::new(),
        Counter::new(),
    ];

    for l in lines {
        for (i, c) in l.chars().enumerate() {
            counters[i].count(c);
        }
    }

    [
        counters
            .iter()
            .map(|c| *(c.nth_most_common(1).unwrap().0))
            .collect(),
        counters
            .iter()
            .map(|c| *(c.in_order().last().unwrap().0))
            .collect(),
    ]
}

fn load_input(name: &str) -> Vec<String> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    reader.lines_unwrap().collect()
}
