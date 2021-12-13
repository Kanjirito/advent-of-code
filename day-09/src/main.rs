use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[allow(clippy::if_same_then_else)]
fn main() {
    let input = load_input();

    // Part 1
    let mut counter: usize = 0;
    for (x, column) in input.iter().enumerate() {
        'number: for (y, number) in column.iter().enumerate() {
            for x_offset in [-1, 0, 1] {
                for y_offset in [-1, 0, 1] {
                    if x_offset == 0 && y_offset == 0 {
                        continue;
                    } else if x == 0 && x_offset == -1 {
                        continue;
                    } else if x == input.len() - 1 && x_offset == 1 {
                        continue;
                    } else if y == 0 && y_offset == -1 {
                        continue;
                    } else if y == column.len() - 1 && y_offset == 1 {
                        continue;
                    } else if number
                        >= &input[(x as isize + x_offset) as usize]
                            [(y as isize + y_offset) as usize]
                    {
                        continue 'number;
                    }
                }
            }

            counter += 1 + number;
        }
    }
    println!("Solution for part 1: {}", counter);
}

fn load_input() -> Vec<Vec<usize>> {
    let file = File::open("input").expect("No input file found");
    let reader = BufReader::new(file);
    let mut input: Vec<Vec<usize>> = Vec::new();
    for line in reader.lines() {
        input.push(line.unwrap().split("").flat_map(|x| x.parse()).collect());
    }
    input
}
