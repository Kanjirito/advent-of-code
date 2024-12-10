use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

fn main() {
    let rules = load_input();
    let start = Cave {
        id: String::from("start"),
        size: Size::Small,
    };

    let mut part1_counter: usize = 0;
    let mut part2_counter: usize = 0;
    let mut small_visited: Vec<Cave> = Vec::new();
    find_path(
        &start,
        &mut small_visited,
        &mut part1_counter,
        &mut part2_counter,
        &rules,
        false,
    );
    println!("Solution for part 1: {}", part1_counter);
    println!("Solution for part 2: {}", part2_counter);
}

fn find_path(
    current_cave: &Cave,
    small_visited: &mut Vec<Cave>,
    part1_counter: &mut usize,
    part2_counter: &mut usize,
    rules: &HashMap<Cave, Vec<Cave>>,
    double_visit: bool,
) {
    if current_cave.is_small() {
        small_visited.push(current_cave.clone());
    };
    for next_cave in &rules[current_cave] {
        match next_cave.size {
            Size::Big => find_path(
                next_cave,
                small_visited,
                part1_counter,
                part2_counter,
                rules,
                double_visit,
            ),
            Size::Small => {
                if small_visited.contains(next_cave) {
                    if double_visit || next_cave.id == "start" {
                        continue;
                    } else {
                        find_path(
                            next_cave,
                            small_visited,
                            part1_counter,
                            part2_counter,
                            rules,
                            true,
                        );
                    }
                } else if next_cave.id == "end" {
                    if !double_visit {
                        *part1_counter += 1;
                    }
                    *part2_counter += 1;
                } else {
                    find_path(
                        next_cave,
                        small_visited,
                        part1_counter,
                        part2_counter,
                        rules,
                        double_visit,
                    );
                }
            }
        }
    }
    if current_cave.is_small() {
        small_visited.pop();
    };
}

fn load_input() -> HashMap<Cave, Vec<Cave>> {
    let file = File::open("input").expect("No input file found");
    let reader = BufReader::new(file);
    let mut rules = HashMap::new();
    for line in reader.lines() {
        let text = line.unwrap();
        let mut caves = text.split('-').map(Cave::from_str);
        let first = caves.next().unwrap().unwrap();
        let second = caves.next().unwrap().unwrap();
        let connections = rules.entry(first.clone()).or_insert_with(Vec::new);
        connections.push(second.clone());
        let connections = rules.entry(second).or_insert_with(Vec::new);
        connections.push(first);
    }
    rules
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Cave {
    id: String,
    size: Size,
}

impl Cave {
    fn is_small(&self) -> bool {
        matches!(self.size, Size::Small)
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
enum Size {
    Small,
    Big,
}

impl FromStr for Cave {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let size = if s.chars().all(|x| x.is_lowercase()) {
            Size::Small
        } else {
            Size::Big
        };
        let id = String::from(s);
        Ok(Self { id, size })
    }
}
