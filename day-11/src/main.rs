use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

fn main() {
    let (input, big_modulo) = load_input();
    println!("Part 1: {}", simulate(input.clone(), 20, true, big_modulo));
    println!("Part 2: {}", simulate(input, 10000, false, big_modulo));
}

fn simulate(mut monkeys: Vec<Monkey>, round_count: u64, divide: bool, big_modulo: u64) -> u64 {
    for _ in 0..round_count {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                let mut new_item = match &monkeys[i].operation {
                    Op::Add(v) => match v {
                        Value::Num(n) => item + n,
                        Value::Old => item + item,
                    },
                    Op::Multi(v) => match v {
                        Value::Num(n) => item * n,
                        Value::Old => item * item,
                    },
                } % big_modulo;
                if divide {
                    new_item /= 3;
                }

                let target = if new_item % monkeys[i].test == 0 {
                    monkeys[i].true_target
                } else {
                    monkeys[i].false_target
                };
                monkeys[target].items.push_back(new_item);
                monkeys[i].inspect_counter += 1;
            }
        }
    }
    monkeys.sort_unstable_by_key(|k| k.inspect_counter);
    monkeys
        .iter()
        .rev()
        .map(|m| m.inspect_counter)
        .take(2)
        .product::<u64>()
}

fn load_input() -> (Vec<Monkey>, u64) {
    let file = File::open("input").expect("No input file found");
    let mut lines = BufReader::new(file).lines();
    let mut monkeys: Vec<Monkey> = Vec::new();
    // Modular arithmetic magic. Basically, if you use the product of all of the numbers you divide against you can use that
    // for modulo operations to reduce the size of the number
    let mut big_modulo = 1;
    while let Some(_) = lines.next() {
        let items: VecDeque<u64> = lines
            .next()
            .unwrap()
            .unwrap()
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|i| i.parse().unwrap())
            .collect();
        let operation: Op = lines.next().unwrap().unwrap().parse().unwrap();
        let test: u64 = lines
            .next()
            .unwrap()
            .unwrap()
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse()
            .unwrap();
        big_modulo *= test;
        let true_target: usize = lines
            .next()
            .unwrap()
            .unwrap()
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();
        let false_target: usize = lines
            .next()
            .unwrap()
            .unwrap()
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();
        monkeys.push(Monkey {
            items,
            operation,
            test,
            false_target,
            true_target,
            inspect_counter: 0,
        });
        // Empty line between monkeys
        lines.next();
    }
    (monkeys, big_modulo)
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Op,
    test: u64,
    true_target: usize,
    false_target: usize,
    inspect_counter: u64,
}

#[derive(Debug, Clone)]
enum Op {
    Add(Value),
    Multi(Value),
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s
            .strip_prefix("  Operation: new = old ")
            .unwrap()
            .split(' ')
            .rev();
        let num: Value = s.next().unwrap().parse().unwrap();
        Ok(match s.next().unwrap() {
            "+" => Self::Add(num),
            "*" => Self::Multi(num),
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Clone)]
enum Value {
    Num(u64),
    Old,
}

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.parse::<u64>() {
            Ok(num) => Self::Num(num),
            Err(_) => Self::Old,
        })
    }
}
