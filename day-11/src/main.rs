use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

fn main() {
    let mut input = load_input();
    let mut counter: usize = 0;
    for step in 0.. {
        let mut already_flashed: HashSet<(usize, usize)> = HashSet::new();
        for x in 1..input.len() - 1 {
            for y in 1..input[0].len() - 1 {
                let tile: &mut Tile = &mut input[x][y];
                if already_flashed.contains(&(x, y)) {
                    continue;
                } else if tile.tick() {
                    already_flashed.insert((x, y));
                    tick_neighbours((x, y), &mut input, &mut already_flashed);
                }
            }
        }
        if step < 100 {
            counter += already_flashed.len();
            if step == 99 {
                println!("Solution for part 1: {}", counter);
            }
        } else if already_flashed.len() == 100 {
            println!("Solution for part 1: {}", step + 1);
            println!();
            pretty_print(&input, false);
            break;
        }
    }
}

fn tick_neighbours(
    cords: (usize, usize),
    grid: &mut [std::vec::Vec<Tile>],
    flashed: &mut HashSet<(usize, usize)>,
) {
    let (x, y) = cords;
    for new_x in x - 1..=x + 1 {
        for new_y in y - 1..=y + 1 {
            let new_cords = (new_x, new_y);
            if new_cords == cords || flashed.contains(&new_cords) {
                continue;
            } else if grid[new_x][new_y].tick() {
                flashed.insert(new_cords);
                tick_neighbours(new_cords, grid, flashed);
            }
        }
    }
}

#[allow(dead_code)]
/// Prints the gird nicely.
fn pretty_print(input: &[Vec<Tile>], border: bool) {
    for line in input {
        let mut str_line = String::new();
        for tile in line {
            match tile {
                Tile::Dumbo(n) => str_line.push(char::from_digit(*n as u32, 10).unwrap()),
                Tile::Border => {
                    if border {
                        str_line.push('X');
                    }
                }
            }
        }
        if !str_line.is_empty() {
            println!("{}", str_line);
        }
    }
}

fn load_input() -> Vec<Vec<Tile>> {
    let file = File::open("input").expect("No input file found");
    let reader = BufReader::new(file);
    let mut input: Vec<Vec<Tile>> = Vec::new();
    for line in reader.lines() {
        let text = format!("x{}x", line.unwrap());
        input.push(text.split("").flat_map(Tile::from_str).collect());
    }
    input.insert(0, vec![Tile::Border; input[0].len()]);
    input.push(vec![Tile::Border; input[0].len()]);
    input
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Dumbo(usize),
    Border,
}

impl Tile {
    /// Ticks the tile. Returns [`true`] if flashed.
    fn tick(&mut self) -> bool {
        match self {
            Self::Border => false,
            Self::Dumbo(value) => {
                *value += 1;
                if *value > 9 {
                    *value = 0;
                    true
                } else {
                    false
                }
            }
        }
    }
}

impl FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "x" {
            return Ok(Self::Border);
        }
        match s.parse::<usize>() {
            Err(_) => Err(()),
            Ok(num) => Ok(Self::Dumbo(num)),
        }
    }
}
