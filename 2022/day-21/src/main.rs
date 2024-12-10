use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

fn main() {
    let input = load_input("input");
    println!("Part 1: {}", get_value(&input, "root"));
    println!("Part 2: {}", part_2(&input));
}

fn part_2(monkeys: &HashMap<String, Monkey>) -> i64 {
    let (first, second) = match &monkeys["root"] {
        Monkey::Number(_) => unreachable!(),
        Monkey::Math(first, _, second) => (first, second),
    };

    let mut path = Vec::new();
    // Try both root values
    let first_val = get_value_or_path(monkeys, first, &mut path);
    let second_val = get_value_or_path(monkeys, second, &mut path);
    // Set the current number to the result of either of them
    let mut counter = first_val.unwrap_or_else(|| second_val.unwrap());

    // Go through the path in reverse order and "undo" each operation
    for (left, symbol, right) in path.iter().rev() {
        match symbol {
            '+' => {
                // For + and * it doesn't matter if the known values is the left or right one
                counter -= left.unwrap_or_else(|| right.unwrap());
            }
            '*' => {
                counter /= left.unwrap_or_else(|| right.unwrap());
            }
            '-' => match left {
                // For both - and / the position of the unknown number matters
                Some(val) => {
                    counter = val - counter;
                }
                None => {
                    counter += right.unwrap();
                }
            },
            '/' => match left {
                Some(val) => {
                    counter = val / counter;
                }
                None => {
                    counter *= right.unwrap();
                }
            },
            _ => unreachable!(),
        }
    }
    counter
}

/// Recursively solve the problem
fn get_value(monkeys: &HashMap<String, Monkey>, name: &str) -> i64 {
    match &monkeys[name] {
        Monkey::Number(num) => *num,
        Monkey::Math(first, symbol, second) => {
            let first_val = get_value(monkeys, first);
            let second_val = get_value(monkeys, second);
            match symbol {
                '+' => first_val + second_val,
                '-' => first_val - second_val,
                '*' => first_val * second_val,
                '/' => first_val / second_val,
                _ => unreachable!(),
            }
        }
    }
}

/// Try to recursively solve the problem and create a path if not possible.
///
/// The basic idea is that you keep trying to solve it but when you get to the human input you return `None` which
/// causes the current operations to be added to a `Vec`.
///
/// The format of each path element is (Option<i64>, char, Option<i64>). A `None` value means that that side relies on human input
/// and can't be determined yet. The side of the unknown values needs to be tracked because it's important for subtractions and divisions.
///
/// After both root values have been checked, the one that returns a number will be used as the base and then each operation from the path
/// will be applied to it but reversed.
fn get_value_or_path(
    monkeys: &HashMap<String, Monkey>,
    name: &str,
    cur_path: &mut Vec<(Option<i64>, char, Option<i64>)>,
) -> Option<i64> {
    // Found the human input, signal to start saving the path
    if name == "humn" {
        return None;
    }
    match &monkeys[name] {
        Monkey::Number(num) => Some(*num),
        Monkey::Math(first, symbol, second) => {
            let first_val = get_value_or_path(monkeys, first, cur_path);
            let second_val = get_value_or_path(monkeys, second, cur_path);
            // If both values are numbers, just solve it normally
            if let (Some(num_1), Some(num_2)) = (first_val, second_val) {
                Some(match symbol {
                    '+' => num_1 + num_2,
                    '-' => num_1 - num_2,
                    '*' => num_1 * num_2,
                    '/' => num_1 / num_2,
                    _ => unreachable!(),
                })
            } else {
                // One of the values found the human input, save the current operation and return None
                cur_path.push((first_val, *symbol, second_val));
                None
            }
        }
    }
}

fn load_input(name: &str) -> HashMap<String, Monkey> {
    let file = File::open(name).expect("No input file found");
    let reader = BufReader::new(file);
    let mut monkeys: HashMap<String, Monkey> = HashMap::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        let (name, job) = line.split_once(": ").unwrap();
        monkeys.insert(name.to_string(), job.parse().unwrap());
    }
    monkeys
}

#[derive(Debug)]
enum Monkey {
    Number(i64),
    Math(String, char, String),
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s.chars().next().unwrap().is_ascii_digit() {
            Self::Number(s.parse().unwrap())
        } else {
            let mut split = s.split(' ');
            Self::Math(
                split.next().unwrap().to_string(),
                split.next().unwrap().chars().next().unwrap(),
                split.next().unwrap().to_string(),
            )
        })
    }
}

#[test]
fn example() {
    let input = load_input("example");
    assert_eq!(get_value(&input, "root"), 152);
    assert_eq!(part_2(&input), 301);
}
