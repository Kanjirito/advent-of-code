use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

fn main() {
    let input = load_input();
    let (count, screen) = solve(&input);
    println!("Part 1: {}", count);
    println!("Part 2:");
    for row in screen {
        println!("{}", String::from_iter(row));
    }
}

fn solve(input: &[Op]) -> (isize, Vec<Vec<char>>) {
    // Counter for part 1
    let mut counter = 0;
    // Check thresholds for part 1
    let mut to_check = 20;
    // Used to delay on add operations
    let mut blocked = false;
    // Current cycle
    let mut cycle = 1;

    let mut register = 1;
    // Operation index
    let mut i = 0;

    let mut screen: Vec<Vec<char>> = vec![vec![' '; 40]; 6];
    // First pixel will not be rendered because cycle 0 doesn't exist but it can be set manually
    // because the register will always be equal to 1 meaning pixel 0 will be drawn
    screen[0][0] = '#';

    while i < input.len() {
        match input[i] {
            Op::NoOP => {
                i += 1;
            }
            Op::Add(num) => {
                if !blocked {
                    // First time an Add operation is ran it only toggles a boolean and doesn't increase the operation index
                    blocked = true;
                } else {
                    // On the second cycle it does the operation and moves the index ahead
                    blocked = false;
                    register += num;
                    i += 1;
                }
            }
        }
        let row: isize = cycle / 40;
        let column: isize = cycle % 40;
        if column.abs_diff(register) <= 1 {
            screen[row as usize][column as usize] = '#';
        }
        cycle += 1;

        if cycle == to_check {
            counter += register * cycle;
            to_check += 40;
        }
    }

    (counter, screen)
}

fn load_input() -> Vec<Op> {
    let file = File::open("input").expect("No input file found");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect()
}

#[derive(Debug)]
enum Op {
    NoOP,
    Add(isize),
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s == "noop" {
            Self::NoOP
        } else {
            Self::Add(s.strip_prefix("addx ").unwrap().parse().unwrap())
        })
    }
}
