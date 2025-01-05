use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use utils::{cursor::*, Grid};

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&input));
}

fn part_1(input: &[Vec<Direction>]) -> String {
    #[rustfmt::skip]
    let key_pad = vec![
        vec!["1", "2", "3"],
        vec!["4", "5", "6"],
        vec!["7", "8", "9"],
    ];
    solve(input, key_pad, (1, 1))
}

fn part_2(input: &[Vec<Direction>]) -> String {
    let key_pad = vec![
        vec!["X", "X", "1", "X", "X"],
        vec!["X", "2", "3", "4", "X"],
        vec!["5", "6", "7", "8", "9"],
        vec!["X", "A", "B", "C", "X"],
        vec!["X", "X", "D", "X", "X"],
    ];
    solve(input, key_pad, (0, 2))
}

fn solve(input: &[Vec<Direction>], key_pad: Grid<&str>, start: (usize, usize)) -> String {
    let mut solution = vec![];
    let mut cur_position = Cursor::new(start.0, start.1, &Direction::CARDINAL);
    for key in input {
        for &step in key {
            cur_position.move_in_direction(|c| *c != "X", &key_pad, step);
        }
        solution.push(*cur_position.index_grid(&key_pad));
    }
    solution.join("")
}

fn load_input(name: &str) -> Vec<Vec<Direction>> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| match c {
                    'U' => Direction::N,
                    'R' => Direction::E,
                    'D' => Direction::S,
                    'L' => Direction::W,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = load_input("example");
        assert_eq!(part_1(&input), "1985")
    }

    #[test]
    fn part_2_test() {
        let input = load_input("example");
        assert_eq!(part_2(&input), "5DB3")
    }
}
