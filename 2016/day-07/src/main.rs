use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;

use utils::BufReadExt;

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&input));
}

fn part_1(lines: &[Vec<Sequence>]) -> u64 {
    let mut counter = 0;

    for l in lines {
        if check_tls(l) {
            counter += 1;
        }
    }

    counter
}

fn part_2(lines: &[Vec<Sequence>]) -> u64 {
    let mut counter = 0;

    for l in lines {
        if check_ssl(l) {
            counter += 1;
        }
    }

    counter
}

fn check_ssl(input: &[Sequence]) -> bool {
    let mut supernet = HashSet::new();
    let mut hypernet = HashSet::new();

    for s in input {
        match s.t {
            Type::Supernet => {
                for x in s.get_aba() {
                    if hypernet.contains(&x) {
                        return true;
                    } else {
                        supernet.insert(x);
                    }
                }
            }
            Type::Hypernet => {
                for mut x in s.get_aba() {
                    x.reverse();
                    if supernet.contains(&x) {
                        return true;
                    } else {
                        hypernet.insert(x);
                    }
                }
            }
        }
    }

    false
}

fn check_tls(input: &[Sequence]) -> bool {
    let mut valid = false;
    for s in input {
        let contains_abba = s.contains_abba();
        match s.t {
            Type::Supernet => {
                if contains_abba {
                    valid = true;
                }
            }
            Type::Hypernet => {
                if contains_abba {
                    return false;
                }
            }
        }
    }
    valid
}

fn load_input(name: &str) -> Vec<Vec<Sequence>> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut lines = vec![];
    for line in reader.lines_unwrap() {
        let mut sequences = vec![];
        let mut cur_sequence = vec![];
        #[allow(clippy::drain_collect)]
        for c in line.chars() {
            if c == '[' {
                sequences.push(Sequence::new_regular(cur_sequence.drain(..).collect()));
            } else if c == ']' {
                sequences.push(Sequence::new_hypernet(cur_sequence.drain(..).collect()));
            } else {
                cur_sequence.push(c);
            }
        }
        sequences.push(Sequence::new_regular(cur_sequence));
        lines.push(sequences);
    }
    lines
}

#[derive(Debug, Clone)]
struct Sequence {
    s: Vec<char>,
    t: Type,
}

impl Sequence {
    fn new_regular(s: Vec<char>) -> Self {
        Self {
            s,
            t: Type::Supernet,
        }
    }

    fn new_hypernet(s: Vec<char>) -> Self {
        Self {
            s,
            t: Type::Hypernet,
        }
    }
    fn contains_abba(&self) -> bool {
        for window in self.s.windows(4) {
            if window[0] == window[3] && window[1] == window[2] && window[0] != window[1] {
                return true;
            }
        }
        false
    }

    fn get_aba(&self) -> Vec<[char; 2]> {
        let mut aba = vec![];
        for window in self.s.windows(3) {
            if window[0] == window[2] && window[0] != window[1] {
                aba.push([window[0], window[1]]);
            }
        }
        aba
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Type {
    Supernet,
    Hypernet,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = load_input("example");
        assert_eq!(part_1(&input), 2);
    }

    #[test]
    fn part_2_test() {
        let input = load_input("example2");
        assert_eq!(part_2(&input), 3);
    }
}
