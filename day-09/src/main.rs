use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const NEIGHBOURS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

fn main() {
    let input = load_input();

    // Part 1
    let mut counter: usize = 0;
    for (x, column) in input.iter().enumerate() {
        'number: for (y, number) in column.iter().enumerate() {
            if number == &9 {
                continue;
            }
            for (x_offset, y_offset) in NEIGHBOURS {
                if number
                    >= &input[(x as isize + x_offset) as usize][(y as isize + y_offset) as usize]
                {
                    continue 'number;
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
        let line: String = format!("{}{}{}", 9, line.unwrap(), 9);
        input.push(line.split("").flat_map(|x| x.parse()).collect());
    }
    let l = input[0].len();
    input.insert(0, vec![9; l]);
    input.push(vec![9; l]);
    input
}
