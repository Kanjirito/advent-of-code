use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input();
    println!("Fully overlapping pairs: {}", part_1(&input));
    println!("Overlapping pairs: {}", part_2(&input));
}

fn part_1(input: &[Pair]) -> usize {
    let mut counter = 0;
    for pair in input {
        if (pair.first.0 <= pair.second.0 && pair.first.1 >= pair.second.1)
            || (pair.first.0 >= pair.second.0 && pair.first.1 <= pair.second.1)
        {
            counter += 1;
        }
    }
    counter
}

fn part_2(input: &[Pair]) -> usize {
    let mut counter = 0;
    for pair in input {
        if !(pair.first.0 > pair.second.1 || pair.first.1 < pair.second.0) {
            counter += 1;
        }
    }
    counter
}

fn load_input() -> Vec<Pair> {
    let file = File::open("input").expect("No input file found");
    let reader = BufReader::new(file);
    let mut result: Vec<Pair> = Vec::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        let mut split = line.split(',');
        let first: Vec<usize> = split
            .next()
            .unwrap()
            .split('-')
            .map(|x| x.parse().unwrap())
            .collect();
        let second: Vec<usize> = split
            .next()
            .unwrap()
            .split('-')
            .map(|x| x.parse().unwrap())
            .collect();
        result.push(Pair {
            first: (first[0], first[1]),
            second: (second[0], second[1]),
        });
    }
    result
}

#[derive(Debug)]
struct Pair {
    first: (usize, usize),
    second: (usize, usize),
}
