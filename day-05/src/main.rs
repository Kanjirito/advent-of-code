use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Rules = HashMap<u64, Vec<u64>>;
type Updates = Vec<Vec<u64>>;

fn main() {
    let (rules, updates) = load_input("input");
    println!("Solution for part 1: {}", part_1(&rules, &updates));
    println!("Solution for part 2: {}", part_2(&rules, &updates));
}

fn part_1(rules: &Rules, updates: &Updates) -> u64 {
    let mut counter = 0;

    for line in updates {
        if is_valid(rules, line) {
            counter += line[line.len() / 2];
        }
    }

    counter
}

fn part_2(rules: &Rules, updates: &Updates) -> u64 {
    let mut counter = 0;

    for line in updates {
        if !is_valid(rules, line) {
            counter += fix_line(rules, line);
        }
    }

    counter
}

fn fix_line(rules: &Rules, line: &[u64]) -> u64 {
    let mut new = vec![];
    for number in line {
        let before = match rules.get(number) {
            Some(b) => b,
            None => {
                new.push(number);
                continue;
            }
        };

        let mut lowest = new.len();
        for b in before {
            if let Some(i) = new.iter().position(|n| *n == b) {
                lowest = lowest.min(i);
            }
        }
        new.insert(lowest, number);
    }
    *new[new.len() / 2]
}

fn is_valid(rules: &Rules, line: &[u64]) -> bool {
    let mut seen: HashSet<u64> = HashSet::new();
    for number in line {
        seen.insert(*number);
        let before = match rules.get(number) {
            Some(b) => b,
            None => continue,
        };
        for before in before {
            if seen.contains(before) {
                return false;
            }
        }
    }
    true
}

fn load_input(name: &str) -> (Rules, Updates) {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut rules: Rules = HashMap::new();
    let mut updates: Updates = vec![];

    let mut lines = reader.lines().map(|l| l.unwrap());

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let mut split = line.split('|');
        rules
            .entry(split.next().unwrap().parse().unwrap())
            .or_default()
            .push(split.next().unwrap().parse().unwrap());
    }

    for line in lines {
        updates.push(line.split(',').map(|s| s.parse().unwrap()).collect());
    }
    (rules, updates)
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let (rules, updates) = load_input("example");
        assert_eq!(part_1(&rules, &updates), 143);
    }

    #[test]
    fn part_2_test() {
        let (rules, updates) = load_input("example");
        assert_eq!(part_2(&rules, &updates), 123);
    }
}
