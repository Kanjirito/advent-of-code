use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
enum Dire {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Dire {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&input));
}

fn part_1(dires: &[Dire]) -> usize {
    let mut houses: HashSet<(isize, isize)> = HashSet::new();
    houses.insert((0, 0));
    let mut x = 0;
    let mut y = 0;
    for d in dires {
        match d {
            Dire::Up => x -= 1,
            Dire::Down => x += 1,
            Dire::Left => y -= 1,
            Dire::Right => y += 1,
        }
        houses.insert((x, y));
    }
    houses.len()
}

fn part_2(dires: &[Dire]) -> usize {
    let mut houses: HashSet<(isize, isize)> = HashSet::new();
    houses.insert((0, 0));
    let mut x_1 = 0;
    let mut x_2 = 0;
    let mut y_1 = 0;
    let mut y_2 = 0;
    for chunk in dires.chunks(2) {
        match chunk[0] {
            Dire::Up => x_1 -= 1,
            Dire::Down => x_1 += 1,
            Dire::Left => y_1 -= 1,
            Dire::Right => y_1 += 1,
        }

        match chunk[1] {
            Dire::Up => x_2 -= 1,
            Dire::Down => x_2 += 1,
            Dire::Left => y_2 -= 1,
            Dire::Right => y_2 += 1,
        }

        houses.insert((x_1, y_1));
        houses.insert((x_2, y_2));
    }

    houses.len()
}

fn load_input(name: &str) -> Vec<Dire> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(Dire::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn helper(ex: &str) -> Vec<Dire> {
        ex.chars().map(Dire::from).collect()
    }

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(&helper("^>v<")), 4);
        assert_eq!(super::part_1(&helper("^v^v^v^v^v")), 2);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(&helper("^v")), 3);
        assert_eq!(super::part_2(&helper("^>v<")), 3);
        assert_eq!(super::part_2(&helper("^v^v^v^v^v")), 11);
    }
}
