use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input();
    let mut total_first: usize = 0;
    let mut total_second: usize = 0;
    for (elf, me) in input {
        total_first += me.fight(&elf);
        match me {
            Move::Rock => {
                total_second += match elf {
                    Move::Rock => Move::Scissors.get_value(),
                    Move::Paper => Move::Rock.get_value(),
                    Move::Scissors => Move::Paper.get_value(),
                };
            }
            Move::Paper => {
                total_second += elf.get_value() + 3;
            }
            Move::Scissors => {
                total_second += 6;
                total_second += match elf {
                    Move::Rock => Move::Paper.get_value(),
                    Move::Paper => Move::Scissors.get_value(),
                    Move::Scissors => Move::Rock.get_value(),
                }
            }
        }
    }
    println!("Total score: {}", total_first);
    println!("Part 2 total score: {}", total_second)
}

fn load_input() -> std::vec::Vec<(Move, Move)> {
    let file = File::open("input").expect("No input file found");
    let reader = BufReader::new(file);
    let mut moves: Vec<(Move, Move)> = Vec::new();
    for line in reader.lines() {
        let x = line.unwrap();
        let mut split = x.split(' ');
        moves.push((split.next().unwrap().into(), split.next().unwrap().into()));
    }
    moves
}

#[derive(Debug)]
enum Move {
    /// Lose
    Rock,
    /// Draw
    Paper,
    /// Win
    Scissors,
}

impl From<&str> for Move {
    fn from(c: &str) -> Self {
        match c {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => unreachable!(),
        }
    }
}

impl Move {
    fn fight(&self, other: &Move) -> usize {
        let mut result: usize = 0;
        match self {
            Move::Rock => {
                result += match other {
                    Move::Rock => 3,
                    Move::Paper => 0,
                    Move::Scissors => 6,
                }
            }
            Move::Paper => {
                result += match other {
                    Move::Rock => 6,
                    Move::Paper => 3,
                    Move::Scissors => 0,
                }
            }
            Move::Scissors => {
                result += match other {
                    Move::Rock => 0,
                    Move::Paper => 6,
                    Move::Scissors => 3,
                }
            }
        }
        result + self.get_value()
    }

    fn get_value(&self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}
