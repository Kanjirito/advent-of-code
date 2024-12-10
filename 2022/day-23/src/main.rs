use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Position = (i64, i64);

fn main() {
    let input = load_input("input");
    let (first, second) = solve(&input);
    println!("Part 1: {}", first);
    println!("Part 2: {}", second);
}

/// Simulates the movements until none are made
fn solve(map: &HashSet<Position>) -> (i64, i64) {
    let mut part_1 = 0;
    let mut cur_map = map.clone();
    let mut checker = Checker::new();
    let mut loop_counter = 1;
    loop {
        // key = target of movement
        // value = Vec of positions that want to move to key
        let mut moves: HashMap<Position, Vec<Position>> = HashMap::new();
        for pos in &cur_map {
            match checker.check_move(*pos, &cur_map) {
                None => {
                    // If no move is planned, "move" to cur position
                    // It's impossible for an elf to try to move to a occupied position so this is safe to do
                    (*moves.entry(*pos).or_default()).push(*pos);
                }
                Some(new_pos) => {
                    (*moves.entry(new_pos).or_default()).push(*pos);
                }
            }
        }
        let mut new_map = HashSet::new();
        for (target, origins) in moves {
            // If multiple elves try to move to the target, don't move them. Instead just add their old position
            if origins.len() > 1 {
                for old in origins {
                    new_map.insert(old);
                }
            } else {
                new_map.insert(target);
            }
        }
        // No change was made check
        if cur_map == new_map {
            break;
        }
        cur_map = new_map;
        checker.rotate_checks();
        if loop_counter == 10 {
            part_1 = count_empty(&cur_map);
        }
        loop_counter += 1;
    }
    (part_1, loop_counter)
}

/// Counts the empty fields in the space that contains all the elves
///
/// Goes over every position and finds the highest and lowest values for each direction.
/// Then simply calculates the surface area needed to contains all the elves and subtracts the elves
fn count_empty(map: &HashSet<Position>) -> i64 {
    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut min_y = i64::MAX;
    let mut max_y = i64::MIN;

    for (x, y) in map {
        min_x = min_x.min(*x);
        max_x = max_x.max(*x);
        min_y = min_y.min(*y);
        max_y = max_y.max(*y);
    }
    // + 1 is needed because 0 index of positions
    ((max_x - min_x + 1) * (max_y - min_y + 1)) - map.len() as i64
}

type Checks<'a> = Vec<&'a dyn Fn(i64, i64, &HashSet<Position>) -> Option<Position>>;

struct Checker<'a> {
    /// List of the checks in the order they should be done
    checks: Checks<'a>,
}

impl Checker<'_> {
    fn new() -> Self {
        let checks: Checks = vec![
            &Self::check_north,
            &Self::check_south,
            &Self::check_west,
            &Self::check_east,
        ];
        Self { checks }
    }

    /// Moves the current first check to the end by simply rotating the Vec that stores the checks
    fn rotate_checks(&mut self) {
        self.checks.rotate_left(1);
    }

    fn check_move(&self, cur: Position, map: &HashSet<Position>) -> Option<Position> {
        let x = cur.0;
        let y = cur.1;
        // Vec that keeps track of results
        let mut empty: Vec<Position> = vec![];
        // Go over every check and call it with the current position
        for f in &self.checks {
            if let Some(result) = f(x, y, map) {
                empty.push(result);
            }
        }
        // If all 4 checks passed that means there's no need to move
        // If none passed then there's nowhere to move
        if empty.len() == 4 || empty.is_empty() {
            None
        } else {
            // Returns the first check that passed.
            Some(empty[0])
        }
    }
    fn check_north(x: i64, y: i64, map: &HashSet<Position>) -> Option<Position> {
        if !map.contains(&(x - 1, y - 1))
            && !map.contains(&(x - 1, y))
            && !map.contains(&(x - 1, y + 1))
        {
            Some((x - 1, y))
        } else {
            None
        }
    }

    fn check_south(x: i64, y: i64, map: &HashSet<Position>) -> Option<Position> {
        if !map.contains(&(x + 1, y - 1))
            && !map.contains(&(x + 1, y))
            && !map.contains(&(x + 1, y + 1))
        {
            Some((x + 1, y))
        } else {
            None
        }
    }

    fn check_west(x: i64, y: i64, map: &HashSet<Position>) -> Option<Position> {
        if !map.contains(&(x - 1, y - 1))
            && !map.contains(&(x, y - 1))
            && !map.contains(&(x + 1, y - 1))
        {
            Some((x, y - 1))
        } else {
            None
        }
    }
    fn check_east(x: i64, y: i64, map: &HashSet<Position>) -> Option<Position> {
        if !map.contains(&(x - 1, y + 1))
            && !map.contains(&(x, y + 1))
            && !map.contains(&(x + 1, y + 1))
        {
            Some((x, y + 1))
        } else {
            None
        }
    }
}

fn load_input(name: &str) -> HashSet<Position> {
    let file = File::open(name).expect("No input file found");
    let reader = BufReader::new(file);
    let mut elves = HashSet::new();
    for (x, line) in reader.lines().map(|l| l.unwrap()).enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert((x as i64, y as i64));
            }
        }
    }
    elves
}

#[test]
fn example() {
    let input = load_input("example");
    let (first, second) = solve(&input);
    assert_eq!(first, 110);
    assert_eq!(second, 20);
}
