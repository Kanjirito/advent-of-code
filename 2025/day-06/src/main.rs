use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;

use utils::BufReadExt;

fn main() {
    let (numbers, operations, input) = load_input("input");
    println!("Solution for part 1: {}", part_1(&numbers, &operations));
    println!("Solution for part 2: {}", part_2(&input));
}

fn part_1(numbers: &[Vec<String>], operations: &[Symbol]) -> u64 {
    let mut counter = 0;

    for (i, operation) in operations.iter().enumerate() {
        let mut cur_counter = match operation {
            Symbol::Add => 0,
            Symbol::Multiply => 1,
        };
        for row in numbers {
            let number: u64 = row[i].parse().unwrap();
            match operation {
                Symbol::Add => cur_counter += number,
                Symbol::Multiply => cur_counter *= number,
            }
        }
        counter += cur_counter;
    }

    counter
}

fn part_2(input: &[Vec<char>]) -> u64 {
    let mut counter: u64 = 0;
    let number_count = input.len() - 1;
    let mut digits: Vec<u64> = vec![];
    let mut symbol = '+';

    for i in 0..input[0].len() {
        if let Some(&s) = input.last().unwrap().get(i)
            && s != ' '
        {
            symbol = s
        }

        let mut cur_digit = String::new();
        for row in &input[0..number_count] {
            cur_digit.push(row[i]);
        }

        match cur_digit.trim().parse() {
            Ok(n) => digits.push(n),
            Err(_) => match symbol {
                '+' => {
                    counter += digits.drain(..).sum::<u64>();
                }
                '*' => {
                    counter += digits.drain(..).product::<u64>();
                }
                _ => unreachable!(),
            },
        }
    }

    counter
}

fn load_input(name: &str) -> (Vec<Vec<String>>, Vec<Symbol>, Vec<Vec<char>>) {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut numbers = vec![];
    let mut symbols = vec![];
    let mut all_text = vec![];

    for mut line in reader.lines_unwrap() {
        if line.starts_with("+") || line.starts_with("*") {
            symbols = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
        } else {
            numbers.push(line.split_whitespace().map(|n| n.to_string()).collect());
        }

        // Empty column at the end to trigger calculations in part 2
        line.push(' ');
        all_text.push(line.chars().collect());
    }

    (numbers, symbols, all_text)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Symbol {
    Add,
    Multiply,
}

impl FromStr for Symbol {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Self::Add,
            "*" => Self::Multiply,
            _ => return Err(()),
        })
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Add => "+",
                Self::Multiply => "*",
            }
        )
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let (numbers, symbols, _) = load_input("example");
        assert_eq!(part_1(&numbers, &symbols), 4277556)
    }

    #[test]
    fn part_2_test() {
        let (_, _, input) = load_input("example");
        assert_eq!(part_2(&input), 3263827)
    }
}
