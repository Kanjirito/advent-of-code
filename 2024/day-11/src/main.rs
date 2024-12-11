use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&input));
}

fn part_1(stones: &[usize]) -> usize {
    let mut cur = count_stones(stones);

    for _ in 0..25 {
        cur = simulate(&cur);
    }
    cur.values().sum()
}

fn part_2(stones: &[usize]) -> usize {
    let mut cur = count_stones(stones);

    for _ in 0..75 {
        cur = simulate(&cur);
    }
    cur.values().sum()
}

fn simulate(stones: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut cur = HashMap::new();

    for (stone, count) in stones {
        if *stone == 0 {
            *cur.entry(1).or_default() += *count;
        } else {
            let n_str = format!("{stone}");
            if n_str.len() % 2 == 0 {
                let (first, second) = n_str.split_at(n_str.len() / 2);
                *cur.entry(first.parse().unwrap()).or_default() += *count;
                *cur.entry(second.parse().unwrap()).or_default() += *count;
            } else {
                *cur.entry(stone * 2024).or_default() += *count;
            }
        }
    }
    cur
}

fn count_stones(stones: &[usize]) -> HashMap<usize, usize> {
    let mut counter = HashMap::new();

    for stone in stones {
        *counter.entry(*stone).or_default() += 1;
    }

    counter
}

fn load_input(name: &str) -> Vec<usize> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    reader.read_line(&mut buf).unwrap();
    buf.trim().split(' ').map(|n| n.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let stones = load_input("example");
        assert_eq!(part_1(&stones), 55312);
    }

    #[test]
    fn part_2_test() {
        let stones = load_input("example");
        assert_eq!(part_2(&stones), 65601038650482);
    }
}
