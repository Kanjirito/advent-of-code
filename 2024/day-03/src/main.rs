use regex::Regex;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&input));
}

fn part_1(input: &[String]) -> u64 {
    let mut result = 0;
    let reg = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for line in input {
        for r in reg.captures_iter(line) {
            let (_, [first, second]) = r.extract();
            result += first.parse::<u64>().unwrap() * second.parse::<u64>().unwrap()
        }
    }
    result
}

fn part_2(input: &[String]) -> u64 {
    let mut result = 0;
    let reg = Regex::new(r"(?:(?:do\(\))|(?:don't\(\)))|(?:mul\((\d{1,3}),(\d{1,3})\))").unwrap();
    let mut allowed = true;
    for line in input {
        for r in reg.captures_iter(line) {
            match r.get(0).unwrap().as_str() {
                "do()" => allowed = true,
                "don't()" => allowed = false,
                _ => {
                    if allowed {
                        result += r.get(1).unwrap().as_str().parse::<u64>().unwrap()
                            * r.get(2).unwrap().as_str().parse::<u64>().unwrap()
                    }
                }
            }
        }
    }
    result
}

fn load_input(name: &str) -> Vec<String> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.unwrap()).collect()
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = load_input("example");
        assert_eq!(part_1(&input), 161);
    }

    #[test]
    fn part_2_test() {
        let input = load_input("example2");
        assert_eq!(part_2(&input), 48);
    }
}
