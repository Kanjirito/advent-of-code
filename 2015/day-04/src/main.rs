use base16ct::lower;
use md5::{Digest, Md5};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", solve(&input, "00000"));
    println!("Solution for part 2: {}", solve(&input, "000000"));
}

fn solve(key: &str, start: &str) -> usize {
    let mut hasher = Md5::new_with_prefix(key);
    for digits in 0.. {
        hasher.update(format!("{}", digits));
        let hash = lower::encode_string(&hasher.finalize_reset());
        if hash.starts_with(start) {
            return digits;
        }
        hasher.update(key);
    }
    unreachable!()
}

fn load_input(name: &str) -> String {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    reader.lines().next().unwrap().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(solve("abcdef", "00000"), 609043);
        assert_eq!(solve("pqrstuv", "00000"), 1048970);
    }
}
