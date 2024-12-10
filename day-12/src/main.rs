use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Position = (usize, usize);
/// The highest possible height is `z` which is `122` unicode so `124` is not possible to be reached.
/// The value is used to create a border around the input to make bounds checking easier.
const TOO_HIGH: u8 = 124;

fn main() {
    let (input, start, end) = load_input("input");
    println!("Part 1: {}", part_1(start, end, &input));
    println!("Part 2: {}", part_2(end, &input));
}

/// BFS search from start to end
fn part_1(start: Position, end: Position, input: &[Vec<u8>]) -> u64 {
    let mut q: VecDeque<(Position, u64)> = VecDeque::new();
    let mut visited: HashSet<Position> = HashSet::new();
    q.push_back((start, 0));
    visited.insert(start);

    while let Some(((x, y), count)) = q.pop_front() {
        let cur = input[x][y];
        if (x, y) == end {
            return count;
        }

        for (i, j) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            if visited.contains(&(i, j)) {
                continue;
            }
            let other = input[i][j];
            if other <= cur + 1 {
                q.push_back(((i, j), count + 1));
                visited.insert((i, j));
            }
        }
    }
    unreachable!()
}

/// Same as part_1 but it returns once it finds a height `a`
fn part_2(start: Position, input: &[Vec<u8>]) -> u64 {
    let mut q: VecDeque<(Position, u64)> = VecDeque::new();
    let mut visited: HashSet<Position> = HashSet::new();
    q.push_back((start, 0));
    visited.insert(start);

    while let Some(((x, y), count)) = q.pop_front() {
        let cur = input[x][y];
        if cur == b'a' {
            return count;
        }

        for (i, j) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            if visited.contains(&(i, j)) {
                continue;
            }
            let other = input[i][j];

            // Other needs to be checked against TOO_HIGH, otherwise bound check will not work
            if other < TOO_HIGH && other >= cur - 1 {
                q.push_back(((i, j), count + 1));
                visited.insert((i, j));
            }
        }
    }
    unreachable!()
}

fn load_input(name: &str) -> (Vec<Vec<u8>>, Position, Position) {
    let file = File::open(name).expect("No input file found");
    let reader = BufReader::new(file);

    let mut start: Position = (0, 0);
    let mut end: Position = (0, 0);
    let mut input = Vec::new();

    for (x, line) in reader.lines().map(|l| l.unwrap()).enumerate() {
        let mut cur_row = Vec::new();
        // Adds left border
        cur_row.push(TOO_HIGH);
        for (y, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start = (x + 1, y + 1);
                    cur_row.push(b'a');
                }
                'E' => {
                    end = (x + 1, y + 1);
                    cur_row.push(b'z');
                }
                _ => cur_row.push(c as u8),
            }
        }
        // Right border
        cur_row.push(TOO_HIGH);
        input.push(cur_row);
    }
    let l = input[0].len();
    // Top border
    input.insert(0, vec![TOO_HIGH; l]);
    // Bottom border
    input.push(vec![TOO_HIGH; l]);
    (input, start, end)
}

#[test]
fn example() {
    let (input, start, end) = load_input("example");
    assert_eq!(start, (1, 1));
    assert_eq!(end, (3, 6));
    assert_eq!(part_1(start, end, &input), 31);
    assert_eq!(part_2(end, &input), 29);
}
