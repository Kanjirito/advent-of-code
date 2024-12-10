#![allow(unused_variables, dead_code)]
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input("input");
    println!(
        "Solution for part 1: {}",
        part_1(input.0.clone(), input.1.clone())
    );
    println!("Solution for part 2: {}", part_2(input.0, input.1));
}

fn part_1(mut first: Vec<u64>, mut second: Vec<u64>) -> u64 {
    let mut distance = 0;
    first.sort_unstable();
    second.sort_unstable();
    for (f, s) in first.into_iter().zip(second.into_iter()) {
        distance += f.abs_diff(s);
    }
    distance
}

fn part_2(first: Vec<u64>, second: Vec<u64>) -> u64 {
    let mut result = 0;
    let mut counter: HashMap<u64, u64> = HashMap::new();

    for s in second {
        *counter.entry(s).or_default() += 1;
    }

    for f in first {
        result += f * counter.get(&f).unwrap_or(&0);
    }

    result
}

fn load_input(name: &str) -> (Vec<u64>, Vec<u64>) {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut first: Vec<u64> = vec![];
    let mut second: Vec<u64> = vec![];

    for line in reader.lines().map(|l| l.unwrap()) {
        let mut split = line.split("   ");
        first.push(split.next().unwrap().parse::<u64>().unwrap());
        second.push(split.next().unwrap().parse::<u64>().unwrap());
    }
    (first, second)
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let (first, second) = load_input("example");
        assert_eq!(part_1(first, second), 11);
    }

    #[test]
    fn part_2_test() {
        let (first, second) = load_input("example");
        assert_eq!(part_2(first, second), 31);
    }
}
