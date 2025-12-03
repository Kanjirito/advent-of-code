#![allow(unused_variables, dead_code)]
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&input));
}

fn part_1(input: &[Vec<u64>]) -> u64 {
    let mut counter = 0;

    for bank in input {
        let mut first: u64 = 0;
        let mut second: u64 = 0;
        for &digit in &bank[..(bank.len() - 1)] {
            if digit > first {
                first = digit;
                second = *bank.last().unwrap();
            } else {
                second = second.max(digit);
            }
        }
        counter += (first * 10) + second;
    }

    counter
}

fn part_2(input: &[Vec<u64>]) -> u64 {
    let mut counter = 0;

    for bank in input {
        // Initializes the array use for tracking. 0 means nothing found yet.
        let mut values = [0; 12];

        // Go over each value in the bank
        for (bank_i, &digit) in bank.iter().enumerate() {

            // Go over every saved value but skip a value if there isn't enough batteries
            // left in the bank.
            // For example:
            // If we are on the 10th battery of a bank that has 15 of them we know that the current
            // battery can't be in the 1st to 7th digit of our solution because there's only 5
            // batteries left.
            for i in 12_usize.saturating_sub(bank.len() - bank_i)..=11 {
                if digit > values[i] {
                    // Found a new biggest battery that we can use
                    values[i] = digit;
                    // Unset every value that comes after it since the values are invalid now
                    values
                        .split_at_mut(i + 1)
                        .1
                        .copy_from_slice(&vec![0; 11 - i]);
                    break;
                }
            }
        }


        // Put the digits together
        let mut cur_number = 0;
        for v in values {
            cur_number *= 10;
            cur_number += v;
        }
        counter += cur_number;
    }

    counter
}

fn load_input(name: &str) -> Vec<Vec<u64>> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut banks = vec![];
    for line in reader.lines().map(|l| l.unwrap()) {
        banks.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect(),
        );
    }
    banks
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = load_input("example");
        assert_eq!(part_1(&input), 357);
    }

    #[test]
    fn part_2_test() {
        let input = load_input("example");
        assert_eq!(part_2(&input), 3121910778619);
    }
}
