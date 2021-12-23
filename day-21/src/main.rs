#![allow(dead_code)]
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let mut players = load_input();
    players.reverse();
    let mut dice = PracticeDice::new(1, 100);
    for roll_n in 1.. {
        let roll = dice.roll();
        let mut current_player = players.pop().unwrap();
        current_player.move_positon(roll);
        if current_player.check_if_won(1000) {
            println!("Solution for part 1: {}", roll_n * 3 * players[0].score);
            break;
        }
        players.insert(0, current_player)
    }
}

fn load_input() -> Vec<Player> {
    let file = File::open("input").expect("No input file found");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|x| {
            Player::new(
                x.unwrap()
                    .split(": ")
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
            )
        })
        .collect()
}

#[derive(Debug)]
struct Player {
    current_position: usize,
    score: usize,
    won: bool,
}

impl Player {
    fn new(current_position: usize) -> Self {
        Self {
            current_position,
            score: 0,
            won: false,
        }
    }

    fn move_positon(&mut self, amount_to_move: usize) {
        let left = (self.current_position + amount_to_move) % 10;
        if left == 0 {
            self.current_position = 10;
        } else {
            self.current_position = left;
        }
        self.score += self.current_position
    }

    fn check_if_won(&mut self, target: usize) -> bool {
        if self.score >= target {
            self.won = true;
            true
        } else {
            false
        }
    }
}

struct PracticeDice {
    current_value: usize,
    min: usize,
    max: usize,
}

impl PracticeDice {
    fn new(min: usize, max: usize) -> Self {
        Self {
            current_value: 1,
            min,
            max,
        }
    }
}

impl Iterator for PracticeDice {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.current_value;
        self.current_value += 1;
        if self.current_value > self.max {
            self.current_value = self.min;
        }
        Some(cur)
    }
}

impl Dice for PracticeDice {}

trait Dice: Iterator<Item = usize> {
    /// Gets the 3 rolls
    fn roll(&mut self) -> usize
    where
        Self: std::marker::Sized,
    {
        self.take(3).sum()
    }
}
