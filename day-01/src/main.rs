use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&input));
}

fn part_1(dire: &[char]) -> isize {
    let mut floor = 0;
    for c in dire {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => unreachable!(),
        }
    }
    floor
}

fn part_2(dire: &[char]) -> usize {
    let mut floor = 0;
    for (i, c) in dire.iter().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => unreachable!(),
        }
        if floor < 0 {
            return i + 1;
        }
    }
    unreachable!()
}

fn load_input(name: &str) -> Vec<char> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let mut reader = BufReader::new(file);
    let mut buff = vec![];
    let _ = reader.read_to_end(&mut buff);
    buff.into_iter()
        .map(std::convert::Into::<char>::into)
        .collect()
}

#[cfg(test)]
#[allow(unused_variables)]
mod tests {

    #[test]
    fn part_1() {
        let input: Vec<_> = "))(((((".chars().collect();
        assert_eq!(super::part_1(&input), 3)
    }

    #[test]
    fn part_2() {
        let input: Vec<_> = "()())".chars().collect();
        assert_eq!(super::part_2(&input), 5)
    }
}
