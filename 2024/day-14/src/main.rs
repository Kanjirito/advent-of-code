#![allow(unused_variables, dead_code)]
use std::cmp::Ordering;
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;

use utils::{print_grid, BufReadExt};

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input, 101, 103));
    part_2(&input, 101, 103);
}

fn part_1(robots: &[Robot], x_size: isize, y_size: isize) -> usize {
    let mut counters = (0, 0, 0, 0);

    for mut robot in robots.iter().cloned() {
        robot.simulate(x_size, y_size, 100);
        match (robot.x.cmp(&(x_size / 2)), robot.y.cmp(&(y_size / 2))) {
            (Ordering::Equal, _) | (_, Ordering::Equal) => {}
            (Ordering::Less, Ordering::Less) => counters.0 += 1,
            (Ordering::Greater, Ordering::Less) => counters.1 += 1,
            (Ordering::Less, Ordering::Greater) => counters.2 += 1,
            (Ordering::Greater, Ordering::Greater) => counters.3 += 1,
        }
    }
    counters.0 * counters.1 * counters.2 * counters.3
}

fn part_2(robots: &[Robot], x_size: isize, y_size: isize) {
    for i in 0..=(x_size * y_size) {
        println!("Iteration: {i}");
        let mut grid = vec![vec![' '; x_size as usize]; y_size as usize];
        for mut robot in robots.iter().cloned() {
            robot.simulate(x_size, y_size, i);
            grid[robot.y as usize][robot.x as usize] = 'X';
        }
        print_grid(&grid);
    }
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    x: isize,
    y: isize,
    v_x: isize,
    v_y: isize,
}

impl Robot {
    fn simulate(&mut self, x_size: isize, y_size: isize, seconds: isize) {
        self.x = (self.x + (self.v_x * seconds)).rem_euclid(x_size);
        self.y = (self.y + (self.v_y * seconds)).rem_euclid(y_size);
    }
}

impl FromStr for Robot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        let (x, y) = split
            .next()
            .unwrap()
            .strip_prefix("p=")
            .unwrap()
            .split_once(',')
            .unwrap();
        let (v_x, v_y) = split
            .next()
            .unwrap()
            .strip_prefix("v=")
            .unwrap()
            .split_once(',')
            .unwrap();
        Ok(Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            v_x: v_x.parse().unwrap(),
            v_y: v_y.parse().unwrap(),
        })
    }
}

fn load_input(name: &str) -> Vec<Robot> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    reader.lines_unwrap().map(|l| l.parse().unwrap()).collect()
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let robots = load_input("example");
        assert_eq!(part_1(&robots, 11, 7), 12);
    }
}
