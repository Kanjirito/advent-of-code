use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input("input");
    let [p1, p2] = solve(&input);
    println!("Solution for part 1: {}", p1);
    println!("Solution for part 2: {}", p2);
}

fn solve(input: &[(Turn, i64)]) -> [i64; 2] {
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut cur_direction = Direction::North;
    let mut visited = HashSet::new();
    let mut first_dup = None;

    for &(t, d) in input {
        cur_direction.turn(t);
        for _ in 0..d {
            match cur_direction {
                Direction::North => y += 1,
                Direction::East => x += 1,
                Direction::South => y -= 1,
                Direction::West => x -= 1,
            };
            if first_dup.is_none() {
                if visited.contains(&(x, y)) {
                    first_dup = Some(x.abs() + y.abs());
                } else {
                    visited.insert((x, y));
                }
            }
        }
    }

    [x.abs() + y.abs(), first_dup.unwrap_or(0)]
}

fn load_input(name: &str) -> Vec<(Turn, i64)> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let mut result = vec![];
    for x in line.trim().split(", ") {
        let turn = match &x[..1] {
            "L" => Turn::Left,
            "R" => Turn::Right,
            _ => unreachable!(),
        };
        let steps = x[1..].parse().unwrap();
        result.push((turn, steps));
    }
    result
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    Right,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn(&mut self, turn: Turn) {
        *self = Self::try_from(
            ((*self as usize)
                + match turn {
                    Turn::Right => 1,
                    Turn::Left => 3,
                })
                % 4,
        )
        .unwrap()
    }
}

impl TryFrom<usize> for Direction {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::North),
            1 => Ok(Self::East),
            2 => Ok(Self::South),
            3 => Ok(Self::West),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use Turn::*;

    #[test]
    fn part_1_test_1() {
        let input = load_input("example");
        assert_eq!(solve(&input)[0], 5);
    }

    #[test]
    fn part_1_test_2() {
        let input = load_input("example2");
        assert_eq!(solve(&input)[0], 2);
    }

    #[test]
    fn part_1_test_3() {
        let input = load_input("example3");
        assert_eq!(solve(&input)[0], 12);
    }

    #[test]
    fn part_2_test() {
        let input = load_input("example4");
        assert_eq!(solve(&input)[1], 4);
    }
}
