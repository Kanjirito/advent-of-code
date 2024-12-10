use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let (map, moves) = load_input("input");
    println!("{}", part_1(&map, &moves));
}

fn part_1(map: &[Vec<Tile>], moves: &[Move]) -> usize {
    let result = solve(map, moves);
    (result.x * 1000) + (result.y * 4) + result.orientation
}

#[allow(dead_code)]
fn print_map(map: &[Vec<Tile>]) {
    for row in map {
        for x in row {
            match x {
                Tile::Empty => print!("."),
                Tile::Wall => print!("#"),
                Tile::Void => print!(" "),
            }
        }
        println!();
    }
}

fn solve(map: &[Vec<Tile>], moves: &[Move]) -> Position {
    let mut position = find_start(map);

    for m in moves {
        match m {
            Move::Left => position.rotate_left(),
            Move::Right => position.rotate_right(),
            Move::Move(count) => {
                for _ in 0..*count {
                    let target = get_target(&position, map);
                    match map[target.0][target.1] {
                        Tile::Empty => {
                            position.x = target.0;
                            position.y = target.1;
                        }
                        Tile::Wall => break,
                        Tile::Void => unreachable!(),
                    }
                }
            }
        }
    }

    position
}

fn get_target(position: &Position, map: &[Vec<Tile>]) -> (usize, usize) {
    let (mut i, mut j) = match position.orientation {
        0 => (position.x, position.y + 1),
        1 => (position.x + 1, position.y),
        2 => (position.x, position.y - 1),
        3 => (position.x - 1, position.y),
        _ => unreachable!(),
    };
    if !map[i][j].not_void() {
        match position.orientation {
            0 => {
                j = map[position.x]
                    .iter()
                    .enumerate()
                    .find(|(_, tile)| tile.not_void())
                    .unwrap()
                    .0
            }
            1 => {
                i = map
                    .iter()
                    .map(|x| &x[j])
                    .enumerate()
                    .find(|(_, t)| t.not_void())
                    .unwrap()
                    .0
            }
            2 => {
                j = map[0].len()
                    - 1
                    - map[position.x]
                        .iter()
                        .rev()
                        .enumerate()
                        .find(|(_, tile)| tile.not_void())
                        .unwrap()
                        .0
            }
            3 => {
                i = map.len()
                    - 1
                    - map
                        .iter()
                        .rev()
                        .map(|x| &x[j])
                        .enumerate()
                        .find(|(_, t)| t.not_void())
                        .unwrap()
                        .0
            }
            _ => unreachable!(),
        }
    }
    (i, j)
}

fn find_start(map: &[Vec<Tile>]) -> Position {
    for (x, row) in map.iter().enumerate() {
        for (y, t) in row.iter().enumerate() {
            if let Tile::Empty = t {
                return Position {
                    x,
                    y,
                    orientation: 0,
                };
            }
        }
    }
    unreachable!()
}

fn load_input(name: &str) -> (Vec<Vec<Tile>>, Vec<Move>) {
    let file = File::open(name).expect("No input file found");
    let reader = BufReader::new(file);
    let mut map: Vec<Vec<Tile>> = Vec::new();
    let mut mapping = true;
    let mut max_len = 0;
    let mut moves: Vec<Move> = Vec::new();

    for line in reader.lines().map(|l| l.unwrap()) {
        if line.is_empty() {
            mapping = false;
        } else if mapping {
            let mut row: Vec<Tile> = vec![Tile::Void];
            row.extend(line.chars().map(|x| x.into()));
            row.push(Tile::Void);
            max_len = max_len.max(row.len());
            map.push(row);
        } else {
            map.insert(0, vec![Tile::Void; max_len]);
            map.push(vec![Tile::Void; max_len]);
            let mut cur_num = String::new();
            for c in line.chars() {
                if c.is_ascii_digit() {
                    cur_num.push(c);
                } else {
                    if !cur_num.is_empty() {
                        moves.push(Move::Move(cur_num.parse().unwrap()));
                        cur_num = String::new();
                    }
                    moves.push(match c {
                        'L' => Move::Left,
                        'R' => Move::Right,
                        _ => unreachable!(),
                    })
                }
            }
            if !cur_num.is_empty() {
                moves.push(Move::Move(cur_num.parse().unwrap()));
            }
        }
    }
    for row in map.iter_mut() {
        let cur_len = row.len();
        if cur_len < max_len {
            row.extend(vec![Tile::Void; max_len - cur_len])
        }
    }
    (map, moves)
}

#[derive(Debug, Clone)]
enum Tile {
    Empty,
    Wall,
    Void,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            ' ' => Self::Void,
            '.' => Self::Empty,
            '#' => Self::Wall,
            _ => unreachable!(),
        }
    }
}

impl Tile {
    fn not_void(&self) -> bool {
        match self {
            Tile::Empty | Tile::Wall => true,
            Tile::Void => false,
        }
    }
}

#[derive(Debug)]
enum Move {
    Left,
    Right,
    Move(u64),
}

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
    /// 0 Right
    ///
    /// 1 Down
    ///
    /// 2 Left
    ///
    /// 3 Up
    orientation: usize,
}

impl Position {
    fn rotate_right(&mut self) {
        self.orientation = (self.orientation + 1) % 4
    }

    fn rotate_left(&mut self) {
        self.orientation = (self.orientation + 3) % 4
    }
}

#[test]
fn example() {
    let (map, moves) = load_input("example");
    assert_eq!(part_1(&map, &moves), 6032);
}
