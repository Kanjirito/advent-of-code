use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

fn main() {
    let moves = load_input();

    // Part 1
    let mut current_position = Position::new();
    follow_moves_simple(&mut current_position, &moves);
    println!("First part solution: {}", current_position.get_result());

    // Part 2
    current_position = Position::new();
    follow_moves_aim(&mut current_position, &moves);
    println!("Second part solution {}", current_position.get_result());
}

#[derive(Debug)]
enum Move {
    Forward,
    Up,
    Down,
}

impl FromStr for Move {
    type Err = ();
    fn from_str(move_str: &str) -> Result<Self, Self::Err> {
        match move_str {
            "forward" => Ok(Self::Forward),
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Position {
    // You can't go back so unsigned is fine
    horizontal: u64,
    // It's a submarine, you can't go above water
    depth: u64,
    // This will break if aim becomes negative but I don't think that can happen
    aim: u64,
}

impl Position {
    fn new() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn get_result(self) -> u64 {
        self.horizontal * self.depth
    }
}

// Part 1 rules
fn follow_moves_simple(postion: &mut Position, moves: &[(Move, u64)]) {
    for (order, amount) in moves {
        match order {
            Move::Forward => postion.horizontal += amount,
            Move::Up => postion.depth -= amount,
            Move::Down => postion.depth += amount,
        }
    }
}

// Part 2 rules
fn follow_moves_aim(position: &mut Position, moves: &[(Move, u64)]) {
    for (order, amount) in moves {
        match order {
            Move::Forward => {
                position.horizontal += amount;
                position.depth += position.aim * amount;
            }
            Move::Up => position.aim -= amount,
            Move::Down => position.aim += amount,
        }
    }
}

fn load_input() -> Vec<(Move, u64)> {
    let file = File::open("input").expect("No input file found");
    let reader = BufReader::new(file);
    let mut orders: Vec<(Move, u64)> = Vec::new();
    for line in reader.lines() {
        match line {
            Err(_) => continue,
            Ok(text) => {
                let mut split_text = text.split(' ');
                let current_move = match Move::from_str(split_text.next().unwrap()) {
                    Err(_) => continue,
                    Ok(move_) => move_,
                };
                let move_amount: u64 = match split_text.next().unwrap().parse() {
                    Err(_) => continue,
                    Ok(num) => num,
                };
                orders.push((current_move, move_amount))
            }
        }
    }

    orders
}
