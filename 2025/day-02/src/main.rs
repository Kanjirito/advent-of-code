use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&input));
}

// Iterates over all the values, discards odd length numbers and then checks
// if the 2 halves are identical.
fn part_1(ranges: &[(u64, u64)]) -> u64 {
    let mut counter = 0;

    for (start, end) in ranges {
        for n in *start..=*end {
            let text = format!("{n}");
            if text.len() % 2 != 0 {
                continue;
            }

            if text[..(text.len() / 2)] == text[(text.len() / 2)..] {
                counter += n;
            }
        }
    }
    counter
}

fn part_2(ranges: &[(u64, u64)]) -> u64 {
    let mut counter = 0;

    for (start, end) in ranges {
        'n_loop: for n in *start..=*end {
            let text = format!("{n}");

            for i in 0..(text.len() / 2) {
                if check_pattern(&text, &text[..=i]) {
                    counter += n;
                    continue 'n_loop;
                }
            }
        }
    }
    counter
}

// Recursively checks if the given string consists of only the given pattern.
fn check_pattern(string: &str, pattern: &str) -> bool {
    let pattern_len = pattern.len();
    if string.is_empty() {
        return true;
    } else if string.len() < pattern_len {
        return false;
    }
    if string.starts_with(pattern) {
        check_pattern(&string[pattern_len..], pattern)
    } else {
        false
    }
}

fn load_input(name: &str) -> Vec<(u64, u64)> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.trim()
        .split(',')
        .map(|r| r.split_once('-').unwrap())
        .map(|(start, end)| (start.parse().unwrap(), end.parse().unwrap()))
        .collect()
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = load_input("example");
        assert_eq!(1227775554, part_1(&input));
    }

    #[test]
    fn part_2_test() {
        let input = load_input("example");
        assert_eq!(4174379265, part_2(&input));
    }
}
