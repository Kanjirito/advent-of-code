use std::fs::File;
use std::io::BufReader;

use utils::BufReadExt;

fn main() {
    let (grid, start) = load_input("input");
    println!("Solution for part 1: {}", part_1(&grid, start));
    println!("Solution for part 2: {}", part_2(&grid, start));
}

fn part_1(grid: &[Vec<Tile>], start: usize) -> usize {
    let mut counter = 0;

    let mut to_split = vec![false; grid[0].len()];
    to_split[start] = true;

    for line in grid[2..].iter().step_by(2) {
        let mut new_to_split = to_split.clone();
        for (i, t) in line.iter().enumerate() {
            if *t == Tile::Splitter && to_split[i] {
                counter += 1;
                new_to_split[i - 1] = true;
                new_to_split[i + 1] = true;
                new_to_split[i] = false
            }
        }
        to_split = new_to_split;
    }

    counter
}

fn part_2(grid: &[Vec<Tile>], start: usize) -> usize {
    let mut to_split = vec![false; grid[0].len()];
    to_split[start] = true;
    let mut timelines = vec![0; grid[0].len()];
    timelines[start] = 1;

    for line in grid[2..].iter().step_by(2) {
        let mut new_to_split = to_split.clone();
        for (i, t) in line.iter().enumerate() {
            if *t == Tile::Splitter && to_split[i] {
                new_to_split[i - 1] = true;
                new_to_split[i + 1] = true;
                new_to_split[i] = false;
                timelines[i + 1] += timelines[i];
                timelines[i - 1] += timelines[i];
                timelines[i] = 0;
            }
        }
        to_split = new_to_split;
    }

    timelines.iter().sum()
}

fn load_input(name: &str) -> (Vec<Vec<Tile>>, usize) {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut grid = vec![];
    let mut start_i = 0;
    for line in reader.lines_unwrap() {
        let mut cur_line = vec![];
        for (i, c) in line.chars().enumerate() {
            let tile = match c {
                'S' => {
                    start_i = i;
                    Tile::Start
                }
                '.' => Tile::Empty,
                '^' => Tile::Splitter,
                _ => unreachable!(),
            };
            cur_line.push(tile);
        }
        grid.push(cur_line);
    }

    (grid, start_i)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Start,
    Empty,
    Splitter,
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let (grid, start) = load_input("example");
        assert_eq!(part_1(&grid, start), 21);
    }

    #[test]
    fn part_2_test() {
        let (grid, start) = load_input("example");
        assert_eq!(part_2(&grid, start), 40);
    }
}
