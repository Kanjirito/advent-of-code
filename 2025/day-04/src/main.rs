#![allow(unused_variables, dead_code)]
use std::fmt;
use std::fs::File;
use std::io::BufReader;

use utils::cursor::{Cursor, Direction};
use utils::{BufReadExt, Grid};

fn main() {
    let mut input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&mut input));
}

fn part_1(grid: &Grid<Tile>) -> usize {
    find_removable(grid).len()
}

fn part_2(grid: &mut Grid<Tile>) -> usize {
    let mut counter = 0;

    loop {
        let to_remove = find_removable(grid);
        if to_remove.is_empty() {
            break;
        }
        counter += to_remove.len();
        for (x, y) in to_remove {
            grid[y][x] = Tile::Empty;
        }
    }
    counter
}

fn find_removable(grid: &Grid<Tile>) -> Vec<(usize, usize)> {
    let mut removable = vec![];
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == Tile::Paper {
                let cursor = Cursor::new(x, y, &Direction::AROUND);
                let count = cursor.get_moves_iter(|t| *t == Tile::Paper, grid).count();
                if count < 4 {
                    removable.push((x, y));
                }
            }
        }
    }
    removable
}

fn load_input(name: &str) -> Grid<Tile> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    reader
        .lines_unwrap()
        .map(|l| l.chars().map(Tile::from).collect())
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Paper,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '@' => Tile::Paper,
            _ => panic!("unexpected value"),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => '.',
                Tile::Paper => '@',
            }
        )
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = load_input("example");
        assert_eq!(part_1(&input), 13);
    }

    #[test]
    fn part_2_test() {
        let mut input = load_input("example");
        assert_eq!(part_2(&mut input), 43);
    }
}
