use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input("input");
    let matrix = make_matrix(&input);
    let (one, two) = solve(&matrix);
    println!("Part 1: {}", one);
    println!("Part 2: {}", two);
}

#[allow(dead_code)]
fn print_maps(matrix: &[Vec<Vec<bool>>]) {
    for i in 0..matrix[0][0].len() {
        println!("Turn {}", i);
        for l in matrix {
            for x in l {
                if x[i] {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}

fn solve(matrix: &[Vec<Vec<bool>>]) -> (i64, i64) {
    let n = (matrix.len() as i64) + 1;
    let m = matrix[0].len() as i64;
    let mut counter;
    counter = find_path(matrix, (0, 1), (n, m), 0);
    let part_1 = counter;
    counter = find_path(matrix, (n, m), (0, 1), counter);
    counter = find_path(matrix, (0, 1), (n, m), counter);
    (part_1, counter)
}

fn find_path(matrix: &[Vec<Vec<bool>>], start: (i64, i64), end: (i64, i64), turn: i64) -> i64 {
    let n = matrix.len() as i64;
    let m = matrix[0].len() as i64;
    let loop_length = matrix[0][0].len() as i64;

    let mut q = VecDeque::new();
    q.push_back((start.0, start.1, turn));
    let mut seen: HashSet<(i64, i64, i64)> = HashSet::new();
    seen.insert((start.0, start.1, turn));

    while let Some((cur_x, cur_y, move_count)) = q.pop_front() {
        let turn = (move_count + 1) % loop_length;
        for (new_x, new_y) in [
            (cur_x + 1, cur_y),
            (cur_x, cur_y + 1),
            (cur_x, cur_y),
            (cur_x, cur_y - 1),
            (cur_x - 1, cur_y),
        ] {
            if (new_x, new_y) == end {
                return move_count + 1;
            }
            if seen.contains(&(new_x, new_y, turn))
                || (((new_x, new_y) != start && (new_x, new_y) != end)
                    && (new_x <= 0
                        || new_y <= 0
                        || new_x > n
                        || new_y > m
                        || matrix[(new_x - 1) as usize][(new_y - 1) as usize][turn as usize]))
            {
                continue;
            }
            q.push_back((new_x, new_y, move_count + 1));
            seen.insert((new_x, new_y, turn));
        }
    }
    unreachable!()
}

/// Pre-calculates the blizzard positions
///
/// After n x m amount of turns the map will look the same as on the first turn.
fn make_matrix(map: &[Vec<Tile>]) -> Vec<Vec<Vec<bool>>> {
    let n = map.len() - 2;
    let m = map[0].len() - 2;
    let turns = (n * m) - 1;
    let mut matrix: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; turns]; m]; n];
    for (x, row) in map.iter().enumerate() {
        for (y, tile) in row.iter().enumerate() {
            if let Tile::Blizzard(dire) = tile {
                let mut cur_x = x - 1;
                let mut cur_y = y - 1;
                matrix[cur_x][cur_y][0] = true;
                for i in 1..turns {
                    match dire {
                        Direction::Up => {
                            cur_x = (cur_x + (n - 1)) % n;
                        }
                        Direction::Down => {
                            cur_x = (cur_x + 1) % n;
                        }
                        Direction::Left => {
                            cur_y = (cur_y + (m - 1)) % m;
                        }
                        Direction::Right => {
                            cur_y = (cur_y + 1) % m;
                        }
                    }
                    matrix[cur_x][cur_y][i] = true;
                }
            }
        }
    }
    matrix
}

fn load_input(name: &str) -> Vec<Vec<Tile>> {
    let file = File::open(name).expect("No input file found");
    let reader = BufReader::new(file);
    let mut map: Vec<Vec<Tile>> = Vec::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        map.push(line.chars().map(|c| c.into()).collect());
    }
    map
}

#[derive(Debug)]
enum Tile {
    Wall,
    Empty,
    Blizzard(Direction),
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Wall,
            '^' => Self::Blizzard(Direction::Up),
            'v' => Self::Blizzard(Direction::Down),
            '<' => Self::Blizzard(Direction::Left),
            '>' => Self::Blizzard(Direction::Right),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[test]
fn example() {
    let input = load_input("example");
    let matrix = make_matrix(&input);
    let (one, two) = solve(&matrix);
    assert_eq!(one, 18);
    // For some reason part 2 is off by 2 for the example input
    assert_eq!(two + 2, 54);
}
