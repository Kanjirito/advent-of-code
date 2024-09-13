use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&input));
}

fn part_1(strings: &[Vec<char>]) -> usize {
    let mut count = 0;
    'outer: for s in strings {
        // Numbers don't show up so it's a safe placeholder
        let mut prev_c = '1';
        let mut vowel_count = 0;
        let mut double = false;
        for c in s {
            if matches!(c, 'a' | 'e' | 'i' | 'o' | 'u') {
                vowel_count += 1;
            }

            if matches!(
                (prev_c, *c),
                ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y')
            ) {
                continue 'outer;
            }

            if prev_c == *c {
                double = true;
            }

            prev_c = *c;
        }
        if double && vowel_count >= 3 {
            count += 1;
        }
    }
    count
}

fn part_2(strings: &[Vec<char>]) -> usize {
    let mut count = 0;
    for s in strings {
        // Numbers are safe placeholders
        let mut last = ['0', '1'];
        let mut pairs: HashMap<(char, char), usize> = HashMap::new();
        let mut double_pair = false;
        let mut repeat = false;
        for (i, c) in s.iter().enumerate() {
            if let Some(last_i) = pairs.get(&(last[1], *c)) {
                // If the previous pair was added on the last index that means it's overlapping
                if *last_i != i - 1 {
                    double_pair = true;
                }
            } else {
                pairs.insert((last[1], *c), i);
            }

            if last[0] == *c {
                repeat = true;
            }
            last.rotate_left(1);
            last[1] = *c;
        }
        if repeat && double_pair {
            count += 1;
        }
    }
    count
}

fn load_input(name: &str) -> Vec<Vec<char>> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let s: Vec<Vec<char>> = [
            "ugknbfddgicrmopn",
            "jchzalrnumimnmhp",
            "haegwjzuvuyypxyu",
            "dvszwmarrgswjxmb",
        ]
        .iter()
        .map(|s| s.chars().collect())
        .collect();
        assert_eq!(part_1(&s), 1)
    }

    #[test]
    fn part_2_nice() {
        let s: Vec<Vec<char>> = ["qjhvhtzxzqqjkmpb", "xxyxx"]
            .iter()
            .map(|s| s.chars().collect())
            .collect();
        assert_eq!(part_2(&s), 2)
    }

    #[test]
    fn part_2_no_repeat() {
        assert_eq!(part_2(&["uurcxstgmygtbstg".chars().collect()]), 0)
    }

    #[test]
    fn part_2_no_double() {
        assert_eq!(part_2(&["ieodomkazucvgmuy".chars().collect()]), 0)
    }

    #[test]
    fn part_2_not_same_repeat() {
        assert_eq!(part_2(&["abxabuiu".chars().collect()]), 1)
    }

    #[test]
    fn part_2_with_4_in_a_row() {
        assert_eq!(part_2(&["aaaa".chars().collect()]), 1)
    }
}
