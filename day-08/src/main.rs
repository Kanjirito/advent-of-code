#![allow(unused_variables, dead_code)]
use itertools::Itertools;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Grid = Vec<Vec<Tile>>;
type Antennas = HashMap<char, Vec<(isize, isize)>>;

fn main() {
    let (grid, antennas) = load_input("input");
    println!("Solution for part 1: {}", part_1(&grid, &antennas));
    println!("Solution for part 2: {}", part_2(&grid, &antennas));
}

fn part_1(grid: &Grid, antennas: &Antennas) -> usize {
    let mut valid = HashSet::new();

    for (name, locations) in antennas {
        for mut combs in locations.iter().combinations(2) {
            let second = combs.pop().unwrap();
            let first = combs.pop().unwrap();

            let x_diff = second.0 - first.0;
            let y_diff = second.1 - first.1;

            let mut tmp_values = vec![first, second];

            for (x_modif, y_modif) in [(x_diff, y_diff), (-x_diff, -y_diff)] {
                let cur = tmp_values.pop().unwrap();
                let new_x = cur.0 + x_modif;
                let new_y = cur.1 + y_modif;

                if (new_x >= 0 && new_x < grid[0].len() as isize)
                    && (new_y >= 0 && new_y < grid.len() as isize)
                {
                    valid.insert((new_x, new_y));
                }
            }
        }
    }
    // println!("{:#?}", valid);
    valid.len()
}

fn part_2(grid: &Grid, antennas: &Antennas) -> usize {
    let mut valid = HashSet::new();

    for (name, locations) in antennas {
        for mut combs in locations.iter().combinations(2) {
            let second = combs.pop().unwrap();
            let first = combs.pop().unwrap();

            let x_diff = second.0 - first.0;
            let y_diff = second.1 - first.1;

            for (x_modif, y_modif) in [(x_diff, y_diff), (-x_diff, -y_diff)] {
                let mut new_x = first.0;
                let mut new_y = first.1;

                while (new_x >= 0 && new_x < grid[0].len() as isize)
                    && (new_y >= 0 && new_y < grid.len() as isize)
                {
                    valid.insert((new_x, new_y));
                    new_x += x_modif;
                    new_y += y_modif;
                }
            }
        }
    }
    valid.len()
}

fn load_input(name: &str) -> (Grid, Antennas) {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut grid = vec![];
    let mut antennas: Antennas = HashMap::new();

    for (y, line) in reader.lines().map(|l| l.unwrap()).enumerate() {
        let mut cur = vec![];
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                cur.push(Tile::Ground);
            } else {
                cur.push(Tile::Antenna(c));
                antennas
                    .entry(c)
                    .or_default()
                    .push((x as isize, y as isize));
            };
        }
        grid.push(cur);
    }
    (grid, antennas)
}

fn print_grid(grid: &Grid) {
    for line in grid {
        for t in line {
            print!("{}", t);
        }
        println!();
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Ground,
    Antenna(char),
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Ground => '.',
                Self::Antenna(c) => *c,
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
        let (grid, antennas) = load_input("example");
        assert_eq!(part_1(&grid, &antennas), 14);
    }

    #[test]
    fn part_2_test() {
        let (grid, antennas) = load_input("example");
        assert_eq!(part_2(&grid, &antennas), 34);
    }

    #[test]
    fn part_2_test_second() {
        let (grid, antennas) = load_input("example2");
        assert_eq!(part_2(&grid, &antennas), 9);
    }
}
