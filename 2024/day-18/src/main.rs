use std::collections::VecDeque;
use std::io::BufReader;
use std::{fmt::Display, fs::File};

use utils::{cursor::*, BufReadExt, Grid, GridMaker};

fn main() {
    let bytes = load_input("input");
    let grid = GridMaker::new_empty(Tile::Corrupted, Tile::Ground, 71, 71);
    println!("Solution for part 1: {}", part_1(grid.clone(), &bytes));
    println!("Solution for part 2: {}", part_2(grid.clone(), &bytes));
    println!(
        "Solution for part 2 binary: {}",
        part_2_binary(grid, &bytes)
    );
}

fn part_1(mut grid: Grid<Tile>, bytes: &[(usize, usize)]) -> usize {
    drop_bytes(&mut grid, &bytes[0..1024]);
    path_find(&grid).unwrap()
}

fn part_2(mut grid: Grid<Tile>, bytes: &[(usize, usize)]) -> String {
    for i in 0..bytes.len() {
        drop_bytes(&mut grid, &bytes[i..=i]);
        if path_find(&grid).is_none() {
            return format!("{},{}", bytes[i].0, bytes[i].1);
        }
    }
    unreachable!()
}

fn part_2_binary(grid: Grid<Tile>, bytes: &[(usize, usize)]) -> String {
    let mut left = 0;
    let mut right = bytes.len() - 1;

    while left < right {
        let mut new_grid = grid.clone();
        let mid = ((right - left) / 2) + left;
        drop_bytes(&mut new_grid, &bytes[..=mid]);
        match path_find(&new_grid) {
            Some(_) => left = mid + 1,
            None => {
                right = mid;
            }
        }
    }
    format!("{},{}", bytes[left].0, bytes[left].1)
}

fn drop_bytes(grid: &mut Grid<Tile>, bytes: &[(usize, usize)]) {
    for &(x, y) in bytes {
        grid[y + 1][x + 1] = Tile::Corrupted;
    }
}

fn path_find(grid: &Grid<Tile>) -> Option<usize> {
    let mut q = VecDeque::new();
    let start = (1, 1);
    let end = (grid.len() - 2, grid.len() - 2);
    q.push_back(Cursor::new(start.0, start.1, &Direction::CARDINAL));

    let mut seen = vec![vec![usize::MAX; grid[0].len()]; grid.len()];
    seen[start.1][start.0] = 0;

    while let Some(cur) = q.pop_front() {
        let (cur_x, cur_y) = cur.get_coords();
        let cur_score = seen[cur_y][cur_x];

        if (cur_x, cur_y) == end {
            return Some(seen[cur_y][cur_x]);
        }

        for next in cur.get_moves_iter(|&t| t != Tile::Corrupted, grid) {
            let (next_x, next_y) = next.get_coords();
            let next_score = seen[next_y][next_x];
            if cur_score + 1 < next_score {
                seen[next_y][next_x] = cur_score + 1;
                q.push_back(next);
            }
        }
    }
    None
}

fn load_input(name: &str) -> Vec<(usize, usize)> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);

    reader
        .lines_unwrap()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Ground,
    Corrupted,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Ground => '.',
                Tile::Corrupted => '#',
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let bytes = load_input("example");
        let mut grid = GridMaker::new_empty(Tile::Corrupted, Tile::Ground, 7, 7);
        drop_bytes(&mut grid, &bytes[0..12]);
        assert_eq!(path_find(&grid), Some(22));
    }

    #[test]
    fn part_2_test() {
        let bytes = load_input("example");
        let grid = GridMaker::new_empty(Tile::Corrupted, Tile::Ground, 7, 7);
        assert_eq!(part_2(grid, &bytes), "6,1");
    }

    #[test]
    fn part_2_test_binary() {
        let bytes = load_input("example");
        let grid = GridMaker::new_empty(Tile::Corrupted, Tile::Ground, 7, 7);
        assert_eq!(part_2_binary(grid, &bytes), "6,1");
    }
}
