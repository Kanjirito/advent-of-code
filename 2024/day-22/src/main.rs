use std::collections::{hash_map::Entry, HashMap};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&input));
}

fn part_1(numbers: &[u64]) -> u64 {
    let mut results = Vec::with_capacity(numbers.len());
    for &n in numbers {
        results.push(simulate(n, 2000).pop().unwrap().0);
    }
    results.into_iter().sum()
}

fn part_2(numbers: &[u64]) -> u64 {
    let mut all_changes: HashMap<[isize; 4], u64> = HashMap::new();
    for &n in numbers {
        let r = simulate(n, 2000);
        let changes = generate_changes(&r);
        for (k, v) in changes {
            *all_changes.entry(k).or_default() += v;
        }
    }
    all_changes.into_values().max().unwrap()
}

fn simulate(mut start: u64, times: usize) -> Vec<(u64, isize)> {
    let mut results = Vec::with_capacity(times + 1);
    results.push((start, 0));
    for _ in 0..times {
        start = generate_new_secret_number(start);
        let diff = start as isize % 10 - results.last().unwrap().0 as isize % 10;
        results.push((start, diff));
    }
    results.remove(0);
    results
}

fn generate_changes(numbers: &[(u64, isize)]) -> HashMap<[isize; 4], u64> {
    let mut map = HashMap::with_capacity(numbers.len());

    for window in numbers.windows(4) {
        let diffs = [window[0].1, window[1].1, window[2].1, window[3].1];
        if let Entry::Vacant(vacant_entry) = map.entry(diffs) {
            vacant_entry.insert(window[3].0 % 10);
        }
    }
    map
}

fn generate_new_secret_number(mut number: u64) -> u64 {
    number = prune(mix(number * 64, number));
    number = prune(mix(number / 32, number));
    number = prune(mix(number * 2048, number));
    number
}

fn mix(value: u64, secret: u64) -> u64 {
    value ^ secret
}

fn prune(secret: u64) -> u64 {
    secret % 16777216
}

fn load_input(name: &str) -> Vec<u64> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.unwrap().parse::<u64>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = load_input("example");
        assert_eq!(part_1(&input), 37327623);
    }

    #[test]
    fn part_2_test() {
        let input = load_input("example2");
        assert_eq!(part_2(&input), 23);
    }
}
