use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &HashMap<String, usize>) -> usize {
    let mut counter = 0;
    for value in input.values() {
        if *value <= 100000 {
            counter += value;
        }
    }
    counter
}

fn part_2(input: &HashMap<String, usize>) -> usize {
    let required: usize = 30000000 - (70000000 - input["/"]);
    let mut min = usize::MAX;
    for value in input.values() {
        if *value >= required {
            min = min.min(*value);
        }
    }
    min
}

fn load_input() -> HashMap<String, usize> {
    let file = File::open("input").expect("No input file found");
    let reader = BufReader::new(file);

    // Stores the current path we are "browsing"
    let mut paths: Vec<String> = Vec::new();
    // Stores the size of a given path
    let mut directories: HashMap<String, usize> = HashMap::new();

    for line in reader.lines().map(|l| l.unwrap()) {
        if line.starts_with("$ cd") {
            let cur_dir = line.strip_prefix("$ cd ").unwrap();
            if cur_dir == ".." {
                paths.pop();
            } else {
                paths.push(cur_dir.to_owned());
            }
        // If the first char is a digit it means it's a file size
        } else if line.chars().next().unwrap().is_ascii_digit() {
            let num: usize = line.split(' ').next().unwrap().parse().unwrap();
            let mut cur_path = String::new();
            // Iterate over every path element and add the file size to it
            for path in paths.iter() {
                cur_path += path;
                *directories.entry(cur_path.clone()).or_insert(0) += num;
            }
        }
    }
    directories
}
