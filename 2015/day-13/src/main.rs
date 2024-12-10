use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use regex::Regex;

type Connection = HashMap<String, isize>;

fn main() {
    let mut input = load_input("input");
    println!("Solution for part 1: {}", solve(&input));
    add_yourself(&mut input);
    println!("Solution for part 2: {}", solve(&input));
}

fn add_yourself(map: &mut HashMap<String, Connection>) {
    let mut yourself = HashMap::new();
    for (k, v) in map.iter_mut() {
        v.insert(String::from("Yourself"), 0);
        yourself.insert(k.clone(), 0);
    }
    map.insert(String::from("Yourself"), yourself);
}

fn solve(map: &HashMap<String, Connection>) -> isize {
    let names: Vec<String> = map.keys().map(|k| k.to_owned()).collect();
    let mut highest = isize::MIN;
    let mut q: Vec<Vec<String>> = names.iter().map(|k| vec![k.clone()]).collect();

    while let Some(mut cur) = q.pop() {
        if cur.len() == names.len() {
            let mut local_count = 0;
            for _ in 0..names.len() {
                let first = &cur[0];
                let second = &cur[1];
                local_count += map[first][second];
                local_count += map[second][first];
                cur.rotate_left(1);
            }
            highest = highest.max(local_count);
        }
        for n in names.iter() {
            if !cur.contains(n) {
                let mut new = cur.clone();
                new.push(n.clone());
                q.push(new);
            }
        }
    }

    highest
}

fn load_input(name: &str) -> HashMap<String, Connection> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let reg =
        Regex::new(r#"(\w+?) would ((?:lose)|(?:gain)) (\d+?) happiness.*to (\w+?)\."#).unwrap();
    let mut map: HashMap<String, Connection> = HashMap::new();

    for line in reader.lines().map(|l| l.unwrap()) {
        let result = reg.captures(&line).unwrap();
        let person = result.get(1).unwrap().as_str().to_owned();
        let mut amount = result.get(3).unwrap().as_str().parse::<isize>().unwrap();
        if result.get(2).unwrap().as_str() == "lose" {
            amount = amount.checked_neg().unwrap();
        }
        let target = result.get(4).unwrap().as_str().to_owned();
        (*map.entry(person).or_default()).insert(target, amount);
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = load_input("example");
        assert_eq!(solve(&input), 330);
    }

    #[test]
    fn part_2_test() {
        let mut input = load_input("example");
        add_yourself(&mut input);
        assert_eq!(solve(&input), 286);
    }
}
