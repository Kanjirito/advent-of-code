use std::cmp::Reverse;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use utils::Counter;

fn main() {
    let mut input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&mut input));
}

fn part_1(rooms: &[Room]) -> u64 {
    rooms.iter().filter(|r| r.is_valid()).map(|r| r.id).sum()
}

fn part_2(rooms: &mut [Room]) -> u64 {
    for r in rooms.iter_mut().filter(|r| r.is_valid()) {
        r.shift_cipher();
        if r.name.contains("pole") {
            return r.id;
        }
    }
    unreachable!()
}

fn load_input(name: &str) -> Vec<Room> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.unwrap().trim().into()).collect()
}

#[derive(Debug)]
struct Room {
    name: String,
    id: u64,
    hash: String,
}

impl Room {
    fn is_valid(&self) -> bool {
        let counter = Counter::from_iter(self.name.chars().filter(|c| c.is_alphabetic()));
        let mut values: Vec<_> = counter.into_iter().collect();
        values.sort_unstable_by_key(|(k, v)| (Reverse(*v), *k));
        let hash: String = values.into_iter().map(|(k, _)| k).take(5).collect();
        self.hash == hash
    }

    fn shift_cipher(&mut self) {
        let mut chars = Vec::with_capacity(self.name.len());
        for c in self.name.chars() {
            if c == '-' {
                chars.push(' ');
            } else {
                chars.push(char::from_u32((c as u32 - 97 + self.id as u32) % 26 + 97).unwrap());
            }
        }
        self.name = chars.into_iter().collect();
    }
}

impl From<&str> for Room {
    fn from(value: &str) -> Self {
        let (name, rest) = value.rsplit_once('-').unwrap();
        let id = rest[0..3].parse().unwrap();
        let hash = rest[4..].strip_suffix(']').unwrap().to_string();
        Self {
            name: name.to_string(),
            id,
            hash,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = load_input("example");
        assert_eq!(part_1(&input), 1514);
    }
}
