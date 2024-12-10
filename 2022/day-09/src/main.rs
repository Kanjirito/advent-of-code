use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

fn main() {
    let input = load_input();
    println!("Part 1: {}", solve(&input, 1));
    println!("Part 2: {}", solve(&input, 9));
}

fn solve(input: &[Move], tail_count: usize) -> usize {
    let mut rope = Rope::new(tail_count);
    for m in input {
        rope.make_move(m);
    }
    rope.visited_last.len()
}

fn load_input() -> std::vec::Vec<Move> {
    let file = File::open("input").expect("No input file found");
    let reader = BufReader::new(file);
    let input: Vec<Move> = reader
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();
    input
}

#[derive(Debug)]
struct Rope {
    knots: Vec<(isize, isize)>,
    visited_last: HashSet<(isize, isize)>,
}

impl Rope {
    fn new(tail_count: usize) -> Self {
        let mut visited = HashSet::new();
        visited.insert((0, 0));
        let knots = vec![(0, 0); tail_count + 1];
        Self {
            knots,
            visited_last: visited,
        }
    }

    fn make_move(&mut self, m: &Move) {
        for _ in 0..m.count {
            match m.dire {
                Direction::Up => {
                    self.knots[0].0 += 1;
                }
                Direction::Down => {
                    self.knots[0].0 -= 1;
                }
                Direction::Left => {
                    self.knots[0].1 -= 1;
                }
                Direction::Right => {
                    self.knots[0].1 += 1;
                }
            }
            self.drag_tail();
        }
    }

    fn drag_tail(&mut self) {
        for i in 1..self.knots.len() {
            let diff = (
                self.knots[i - 1].0 - self.knots[i].0,
                self.knots[i - 1].1 - self.knots[i].1,
            );
            let knot = &mut self.knots[i];

            // See image in the directory for explanations
            match diff {
                (2, -1) | (1, -2) | (2, -2) => {
                    knot.0 += 1;
                    knot.1 -= 1;
                }
                (2, 0) => {
                    knot.0 += 1;
                }
                (2, 1) | (1, 2) | (2, 2) => {
                    knot.0 += 1;
                    knot.1 += 1;
                }
                (0, 2) => {
                    knot.1 += 1;
                }
                (-1, 2) | (-2, 1) | (-2, 2) => {
                    knot.0 -= 1;
                    knot.1 += 1;
                }
                (-2, 0) => {
                    knot.0 -= 1;
                }
                (-2, -1) | (-1, -2) | (-2, -2) => {
                    knot.0 -= 1;
                    knot.1 -= 1;
                }
                (0, -2) => {
                    knot.1 -= 1;
                }
                (_, _) => {}
            }
        }
        self.visited_last.insert(*self.knots.iter().last().unwrap());
    }
}

#[derive(Debug)]
struct Move {
    dire: Direction,
    count: isize,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        let dire = match split.next().unwrap() {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => unreachable!(),
        };
        let count: isize = split.next().unwrap().parse().unwrap();
        Ok(Self { dire, count })
    }
}
