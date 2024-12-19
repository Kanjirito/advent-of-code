use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;

use utils::BufReadExt;

fn main() {
    let (stripes, designs) = load_input("input");
    println!("Solution for part 1: {}", part_1(&stripes, &designs));
    println!("Solution for part 2: {}", part_2(&stripes, &designs));
}

fn part_1(stripes: &[String], designs: &[String]) -> usize {
    let mut counter = 0;
    let stripes: HashSet<String> = HashSet::from_iter(stripes.iter().cloned());
    let max_len = stripes.iter().map(|x| x.len()).max().unwrap();
    let mut cache: HashMap<&str, usize> = HashMap::new();
    cache.insert("", 1);

    for design in designs {
        if solve(&stripes, design, max_len, &mut cache) > 0 {
            counter += 1;
        }
    }
    counter
}

fn part_2(stripes: &[String], designs: &[String]) -> usize {
    let mut counter = 0;
    let stripes: HashSet<String> = HashSet::from_iter(stripes.iter().cloned());
    let max_len = stripes.iter().map(|x| x.len()).max().unwrap();
    let mut cache: HashMap<&str, usize> = HashMap::new();
    cache.insert("", 1);

    for design in designs {
        counter += solve(&stripes, design, max_len, &mut cache);
    }
    counter
}

fn solve<'a>(
    stripes: &HashSet<String>,
    to_check: &'a str,
    max_len: usize,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(&old) = cache.get(to_check) {
        return old;
    }
    let mut counter = 0;
    for i in 0..(to_check.len().min(max_len)) {
        if !stripes.contains(&to_check[0..=i]) {
            continue;
        }
        counter += solve(stripes, &to_check[i + 1..], max_len, cache);
    }
    cache.entry(to_check).or_insert(counter);
    counter
}

fn load_input(name: &str) -> (Vec<String>, Vec<String>) {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut lines = reader.lines_unwrap();

    let stripes = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect();
    lines.next();
    let designs = lines.collect();
    (stripes, designs)
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let (stripes, designs) = load_input("example");
        assert_eq!(part_1(&stripes, &designs), 6);
    }

    #[test]
    fn part_2_test() {
        let (stripes, designs) = load_input("example");
        assert_eq!(part_2(&stripes, &designs), 16);
    }
}
