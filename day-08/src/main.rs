use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&input));
}

fn part_1(strings: &[String]) -> usize {
    let mut counter = 0;
    let hex_reg = Regex::new(r"\\x[[:xdigit:]][[:xdigit:]]").unwrap();
    for s in strings {
        let mut cur = s
            .strip_prefix('"')
            .unwrap()
            .strip_suffix('"')
            .unwrap()
            .to_owned();
        cur = cur.replace("\\\\", "\\");
        cur = hex_reg.replace_all(&cur, "|").to_string();
        cur = cur.replace("\\\"", "\"");
        counter += s.len() - cur.len();
    }
    counter
}

#[allow(dead_code)]
fn part_1_no_replace(strings: &[String]) -> usize {
    let mut counter = 0;
    let hex_reg = Regex::new(r"\\x[[:xdigit:]][[:xdigit:]]").unwrap();
    let double_slash_reg = Regex::new(r"\\\\").unwrap();
    let qoute_reg = Regex::new(r#"\\\""#).unwrap();

    for s in strings {
        let mut local = s.len() - 2;
        local -= double_slash_reg.find_iter(s).count();
        local -= qoute_reg.find_iter(s).count();
        local -= hex_reg.find_iter(s).map(|_| 3).sum::<usize>();
        counter += s.len() - local;
    }

    counter
}

fn part_2(strings: &[String]) -> usize {
    let mut counter = 0;
    for s in strings {
        // Every string gets surrounded by quotes
        let mut local = 2;
        for c in s.chars() {
            match c {
                '"' | '\\' => local += 2,
                _ => local += 1,
            }
        }
        counter += local - s.len();
    }
    counter
}

fn load_input(name: &str) -> Vec<String> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = load_input("example");
        assert_eq!(part_1(&input), 12);
    }

    #[test]
    fn part_1_no_replace_test() {
        let input = load_input("example");
        assert_eq!(part_1_no_replace(&input), 12);
    }

    #[test]
    fn part_2_test() {
        let input = load_input("example");
        assert_eq!(part_2(&input), 19);
    }
}
