use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;
use std::io::BufReader;
use std::{fs::File, io::BufRead};

use utils::Grid;

fn main() {
    let input = load_input("input");
    let (score, chair) = path_find(&input);
    println!("Solution for part 1: {}", score);
    println!("Solution for part 2: {}", chair);
}

fn path_find(maze: &Grid<Tile>) -> (usize, usize) {
    let start = Value::new(1, maze.len() - 2, Direction::Right);
    let end = (maze[0].len() - 2, 1);
    let mut q = BinaryHeap::new();
    q.push(start);

    let mut seen: HashMap<Position, (Vec<Position>, usize)> = HashMap::new();
    seen.insert(start.pos, (vec![start.pos], 0));

    while let Some(cur_value) = q.pop() {
        if (cur_value.pos.x, cur_value.pos.y) == end {
            let cur_path = seen.remove(&cur_value.pos).unwrap().0;
            let paths = HashSet::<(usize, usize)>::from_iter(cur_path.iter().map(|p| (p.x, p.y)));
            return (cur_value.cost, paths.len());
        }

        for next in cur_value.get_next() {
            if let Tile::Wall = maze[next.pos.y][next.pos.x] {
                continue;
            }
            let mut cur_path = seen.get(&cur_value.pos).unwrap().0.clone();
            if let Some((seen_path, seen_cost)) = seen.get_mut(&next.pos) {
                match next.cost.cmp(seen_cost) {
                    Ordering::Less => {}
                    Ordering::Equal => {
                        seen_path.extend(cur_path);
                        continue;
                    }
                    Ordering::Greater => continue,
                }
            }

            cur_path.push(next.pos);
            seen.insert(next.pos, (cur_path, next.cost));
            q.push(next);
        }
    }

    unreachable!()
}

#[allow(dead_code)]
fn print_visited(maze: &Grid<Tile>, path: &HashSet<(usize, usize)>) {
    let mut maze = maze.to_vec();

    for (x, y) in path {
        maze[*y][*x] = Tile::Visited;
    }
    utils::print_grid(&maze);
}

fn load_input(name: &str) -> Grid<Tile> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| Tile::try_from(c).unwrap())
                .collect()
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
    dire: Direction,
}

#[derive(Debug, Clone, Copy)]
struct Value {
    pos: Position,
    cost: usize,
}

impl Value {
    fn new(x: usize, y: usize, dire: Direction) -> Self {
        let pos = Position { x, y, dire };
        Self { pos, cost: 0 }
    }

    fn rotate_left(&self) -> Self {
        let dire = match self.pos.dire {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        };
        Self {
            pos: Position {
                x: self.pos.x,
                y: self.pos.y,
                dire,
            },
            cost: self.cost + 1000,
        }
    }

    fn rotate_right(&self) -> Self {
        let dire = match self.pos.dire {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
        Self {
            pos: Position {
                x: self.pos.x,
                y: self.pos.y,
                dire,
            },
            cost: self.cost + 1000,
        }
    }

    fn move_ahead(&self) -> Self {
        let (x, y) = match self.pos.dire {
            Direction::Up => (self.pos.x, self.pos.y - 1),
            Direction::Right => (self.pos.x + 1, self.pos.y),
            Direction::Down => (self.pos.x, self.pos.y + 1),
            Direction::Left => (self.pos.x - 1, self.pos.y),
        };
        let pos = Position {
            x,
            y,
            dire: self.pos.dire,
        };
        Self {
            pos,
            cost: self.cost + 1,
        }
    }

    fn get_next(&self) -> [Self; 3] {
        [self.rotate_left(), self.move_ahead(), self.rotate_right()]
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Value {}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Ground,
    Wall,
    Visited,
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' | 'S' | 'E' => Ok(Self::Ground),
            '#' => Ok(Self::Wall),
            _ => Err(()),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Ground => '.',
                Tile::Wall => '#',
                Tile::Visited => '0',
            }
        )
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test_1() {
        let maze = load_input("example");
        assert_eq!(path_find(&maze).0, 7036);
    }

    #[test]
    fn part_1_test_2() {
        let maze = load_input("example2");
        assert_eq!(path_find(&maze).0, 11048);
    }

    #[test]
    fn part_2_test_1() {
        let maze = load_input("example");
        assert_eq!(path_find(&maze).1, 45);
    }

    #[test]
    fn part_2_test_2() {
        let maze = load_input("example2");
        assert_eq!(path_find(&maze).1, 64);
    }
}
