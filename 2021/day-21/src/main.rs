use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

// All the possible rolls from the quantum dice
const QUANTUM: [usize; 27] = [
    3, 4, 4, 4, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 8, 8, 8, 9,
];

fn main() {
    let mut players = load_input();
    let first_state = GameState {
        player_one: players[0],
        player_two: players[1],
    };
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
    println!(
        "Solution for part 2: {}",
        solve_from_state(first_state, &mut HashMap::new())
            .iter()
            .max()
            .unwrap()
    );
}

fn solve_from_state(state: GameState, cache: &mut HashMap<GameState, [usize; 2]>) -> [usize; 2] {
    if let Some(counters) = cache.get(&state) {
        return *counters;
    }

    let mut local_counters = [0, 0];
    'first: for first_roll in QUANTUM {
        for second_roll in QUANTUM {
            let mut new_state = state;
            new_state.player_one.move_positon(first_roll);
            new_state.player_two.move_positon(second_roll);
            if new_state.player_one.check_if_won(21) {
                local_counters[0] += 1;
                // Player 1 already won so stop searching for player 2 wins
                continue 'first;
            } else if new_state.player_two.check_if_won(21) {
                local_counters[1] += 1;
            } else {
                let r = solve_from_state(new_state, cache);
                local_counters[0] += r[0];
                local_counters[1] += r[1];
            }
        }
    }
    cache.insert(state, local_counters);
    local_counters
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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

    fn check_if_won(&self, target: usize) -> bool {
        self.score >= target
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct GameState {
    player_one: Player,
    player_two: Player,
}
