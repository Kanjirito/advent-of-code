#![allow(dead_code)]
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input();
    let mut fish_counter: Vec<usize> = vec![0; 9];

    for number in &input {
        fish_counter[*number] += 1;
    }
    println!(
        "Solution for part 1: {}",
        simulate_efficient(80, fish_counter.clone())
    );
    println!(
        "Solution for part 2: {}",
        simulate_efficient(256, fish_counter)
    );
}

fn simulate_efficient(days: usize, mut fishes: Vec<usize>) -> usize {
    for _ in 0..days {
        let mut new_fishes: Vec<usize> = vec![0; 9];
        for (i, fish) in fishes.iter().enumerate() {
            if i == 0 {
                new_fishes[6] += *fish;
                new_fishes[8] += *fish;
            } else {
                new_fishes[i - 1] += *fish;
            };
        }
        fishes = new_fishes.clone();
    }
    fishes.iter().sum()
}

/// This will work... someday...
fn turn_on_heater(input: &[usize]) {
    let mut fishes: Vec<Fish> = Vec::new();
    for number in input {
        fishes.push(Fish::from_data(*number));
    }
    println!("Solution for part 1: {}", simulate(80, fishes.clone()));
    println!("Solution for part 2: {}", simulate(256, fishes));
}

fn simulate(days: usize, mut fishes: Vec<Fish>) -> usize {
    for _ in 0..days {
        let mut new_fish_counter: usize = 0;
        for fish in &mut fishes {
            if fish.age_tick() {
                new_fish_counter += 1;
            }
        }

        for _ in 0..new_fish_counter {
            fishes.push(Fish::new())
        }
    }
    fishes.len()
}

fn load_input() -> Vec<usize> {
    let file = File::open("input").expect("No input file found");
    let reader = BufReader::new(file);
    reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

#[derive(Clone)]
struct Fish {
    counter: usize,
}

impl Fish {
    fn new() -> Self {
        Self { counter: 8 }
    }
    fn from_data(counter: usize) -> Self {
        Self { counter }
    }

    /// Decrements the age and returns true if the counter reached 0
    fn age_tick(&mut self) -> bool {
        match self.counter {
            0 => {
                self.counter = 6;
                true
            }
            _ => {
                self.counter -= 1;
                false
            }
        }
    }
}
