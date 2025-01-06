use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use md5::{Digest, Md5};

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&input));
}

fn part_1(password: &str) -> String {
    let mut solution: Vec<char> = vec![];
    for i in 0.. {
        let hash = base16ct::lower::encode_string(&Md5::digest(format!("{password}{i}")));
        if hash.starts_with("00000") {
            solution.push(hash.chars().nth(5).unwrap());
            if solution.len() == 8 {
                break;
            }
        }
    }

    solution.into_iter().collect()
}

fn part_2(password: &str) -> String {
    let mut solution: [char; 8] = ['x'; 8];
    let mut counter = [false; 8];
    for i in 0.. {
        let hash = base16ct::lower::encode_string(&Md5::digest(format!("{password}{i}")));
        if hash.starts_with("00000") {
            let mut chars = hash.chars().skip(5);
            let position = match chars.next().unwrap().to_digit(10) {
                Some(n) => {
                    if n >= 8 {
                        continue;
                    } else {
                        n as usize
                    }
                }
                None => continue,
            };
            if !counter[position] {
                solution[position] = chars.next().unwrap();
                counter[position] = true;
            }
            if counter.iter().all(|b| *b) {
                break;
            }
        }
    }

    solution.into_iter().collect()
}

fn load_input(name: &str) -> String {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    reader.lines().next().unwrap().unwrap()
}
