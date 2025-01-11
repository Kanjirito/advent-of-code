use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::BufReader;

use regex::Regex;
use utils::BufReadExt;

fn main() {
    let (bots, moves) = load_input("input");

    println!("Solution for part 1: {}", part_1(bots.clone(), &moves));
    println!("Solution for part 2: {}", part_2(bots, &moves));
}

fn part_1(mut bots: HashMap<u64, Bot>, moves: &[Move]) -> u64 {
    let mut q = VecDeque::from_iter(moves.iter().cloned());

    while let Some(m) = q.pop_front() {
        match m.target {
            Target::Bot(t) => match bots.get_mut(&t).unwrap().push_number(m.value) {
                Some(x) => {
                    if x[0].value == 17 && x[1].value == 61 {
                        return t;
                    }
                    q.extend(x);
                }
                None => continue,
            },
            Target::Output(_) => continue,
        };
    }
    unreachable!()
}

fn part_2(mut bots: HashMap<u64, Bot>, moves: &[Move]) -> u64 {
    let mut q = VecDeque::from_iter(moves.iter().cloned());
    let mut outputs = HashMap::new();

    while let Some(m) = q.pop_front() {
        match m.target {
            Target::Bot(t) => match bots.get_mut(&t).unwrap().push_number(m.value) {
                Some(x) => {
                    q.extend(x);
                }
                None => continue,
            },
            Target::Output(t) => {
                outputs.insert(t, m.value);
            }
        };
    }
    outputs.get(&0).unwrap() * outputs.get(&1).unwrap() * outputs.get(&2).unwrap()
}

fn load_input(name: &str) -> (HashMap<u64, Bot>, Vec<Move>) {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut bots = HashMap::new();
    let mut moves = vec![];

    let re_bot =
        Regex::new(r#"bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)"#)
            .unwrap();
    let value_reg = Regex::new(r#"value (\d+) goes to bot (\d+)"#).unwrap();

    for line in reader.lines_unwrap() {
        if line.starts_with("bot") {
            let captures = re_bot.captures(&line).unwrap();
            let id: u64 = captures.get(1).unwrap().as_str().parse().unwrap();
            let l_target = captures.get(3).unwrap().as_str().parse().unwrap();
            let h_target = captures.get(5).unwrap().as_str().parse().unwrap();

            let low_target = match captures.get(2).unwrap().as_str() {
                "bot" => Target::Bot(l_target),
                "output" => Target::Output(l_target),
                _ => unreachable!(),
            };

            let high_target = match captures.get(4).unwrap().as_str() {
                "bot" => Target::Bot(h_target),
                "output" => Target::Output(h_target),
                _ => unreachable!(),
            };

            bots.insert(
                id,
                Bot {
                    low_target,
                    high_target,
                    low: None,
                    high: None,
                },
            );
        } else {
            let captures = value_reg.captures(&line).unwrap();
            let value = captures.get(1).unwrap().as_str().parse().unwrap();
            let target = Target::Bot(captures.get(2).unwrap().as_str().parse().unwrap());
            moves.push(Move { target, value });
        }
    }

    (bots, moves)
}

#[derive(Debug, Clone, Copy)]
struct Move {
    value: u64,
    target: Target,
}

#[derive(Debug, Clone, Copy)]
enum Target {
    Bot(u64),
    Output(u64),
}

#[derive(Debug, Clone, Copy)]
struct Bot {
    low_target: Target,
    high_target: Target,
    low: Option<u64>,
    high: Option<u64>,
}

impl Bot {
    fn push_number(&mut self, number: u64) -> Option<[Move; 2]> {
        match self.low {
            Some(v) => {
                if v > number {
                    self.low = Some(number);
                    self.high = Some(v);
                } else {
                    self.high = Some(number);
                }
            }
            None => self.low = Some(number),
        }
        match (self.low, self.high) {
            (Some(l), Some(h)) => {
                self.low = None;
                self.high = None;
                Some([
                    Move {
                        value: l,
                        target: self.low_target,
                    },
                    Move {
                        value: h,
                        target: self.high_target,
                    },
                ])
            }
            _ => None,
        }
    }
}
