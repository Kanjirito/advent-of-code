use std::fs::File;
use std::io::BufReader;

use utils::BufReadExt;

type Screen = [[bool; 50]; 6];

fn main() {
    let input = load_input("input");
    println!("Solution for part 2:");
    println!("Solution for part 1: {}", part_1(&input));
}

fn part_1(instructions: &[Instruction]) -> usize {
    let mut screen = [[false; 50]; 6];

    for &i in instructions {
        match i {
            Instruction::Rect(columns, rows) =>
            {
                #[allow(clippy::needless_range_loop)]
                for y in 0..rows {
                    for x in 0..columns {
                        screen[y][x] = true;
                    }
                }
            }
            Instruction::Row(row, by) => {
                screen[row].rotate_right(by);
            }
            Instruction::Column(column, by) => {
                rotate_column(&mut screen, column, by);
            }
        }
    }
    print_screen(&screen);

    screen
        .into_iter()
        .flat_map(|r| r.into_iter())
        .filter(|x| *x)
        .count()
}

fn print_screen(screen: &Screen) {
    for row in screen {
        for p in row {
            match p {
                true => print!("#"),
                false => print!(" "),
            }
        }
        println!();
    }
    println!()
}

fn rotate_column(grid: &mut Screen, column: usize, by: usize) {
    let mut tmp = [false; 6];
    for (i, row) in grid.iter().enumerate() {
        tmp[i] = row[column]
    }
    tmp.rotate_right(by);

    for (i, x) in tmp.into_iter().enumerate() {
        grid[i][column] = x;
    }
}

fn load_input(name: &str) -> Vec<Instruction> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    reader.lines_unwrap().map(Instruction::from).collect()
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Rect(usize, usize),
    Row(usize, usize),
    Column(usize, usize),
}

impl From<String> for Instruction {
    fn from(value: String) -> Self {
        if let Some(rest) = value.strip_prefix("rect ") {
            let (x, y) = rest.split_once('x').unwrap();
            Self::Rect(x.parse().unwrap(), y.parse().unwrap())
        } else if let Some(rest) = value.strip_prefix("rotate row ") {
            let (y, by) = rest[2..].split_once(' ').unwrap();
            Self::Row(y.parse().unwrap(), by[3..].parse().unwrap())
        } else if let Some(rest) = value.strip_prefix("rotate column ") {
            let (y, by) = rest[2..].split_once(' ').unwrap();
            Self::Column(y.parse().unwrap(), by[3..].parse().unwrap())
        } else {
            unreachable!()
        }
    }
}
