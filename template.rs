#![allow(unused_variables, dead_code)]
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input("input");
    // println!("Solution for part 1: {}", part_1(&input));
    // println!("Solution for part 2: {}", part_2(&input));
}

fn part_1() {
    todo!()
}

fn part_2() {
    todo!()
}

fn load_input(name: &str) {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    todo!()
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        todo!()
    }

    #[test]
    fn part_2_test() {
        todo!()
    }
}
