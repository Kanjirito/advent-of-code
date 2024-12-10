use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&input));
}

fn part_1(lines: &[Vec<u64>]) -> u64 {
    lines.iter().filter(|l| solve(l, &[])).count() as u64
}

fn part_2(lines: &[Vec<u64>]) -> u64 {
    let mut counter = 0;
    for line in lines {
        for i in 0..line.len() {
            if solve(&line[0..i], &line[i + 1..]) {
                counter += 1;
                break;
            }
        }
    }
    counter
}

fn solve(before: &[u64], after: &[u64]) -> bool {
    let mut line = before.iter().chain(after);
    let mut ord = Order::None;
    let mut prev = line.next().unwrap();
    for cur in line {
        let diff = cur.abs_diff(*prev);
        if !(1..=3).contains(&diff) {
            return false;
        }
        let cmp = cur.cmp(prev);
        match (ord, cmp) {
            (Order::None, o) => ord = o.into(),
            (Order::Decreasing, std::cmp::Ordering::Less)
            | (Order::Increasing, std::cmp::Ordering::Greater) => {}
            _ => {
                return false;
            }
        }
        prev = cur;
    }
    true
}

#[derive(Debug, Clone, Copy)]
enum Order {
    Increasing,
    Decreasing,
    None,
}

impl From<std::cmp::Ordering> for Order {
    fn from(value: std::cmp::Ordering) -> Self {
        match value {
            std::cmp::Ordering::Less => Self::Decreasing,
            std::cmp::Ordering::Equal => unreachable!(),
            std::cmp::Ordering::Greater => Self::Increasing,
        }
    }
}

fn load_input(name: &str) -> Vec<Vec<u64>> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut lines = vec![];
    for line in reader.lines().map(|l| l.unwrap()) {
        lines.push(line.split(' ').map(|n| n.parse::<u64>().unwrap()).collect())
    }
    lines
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = load_input("example");
        assert_eq!(part_1(&input), 2);
    }

    #[test]
    fn part_2_test() {
        let input = load_input("example");
        assert_eq!(part_2(&input), 4);
    }
}
