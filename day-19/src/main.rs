use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Rules = HashMap<String, Vec<String>>;

fn main() {
    let (rules, start) = load_input("input");
    println!("Solution for part 1: {}", part_1(&rules, &start));
    // println!("Solution for part 2: {}", part_2(&rules, &start));
}

fn part_1(rules: &Rules, start: &str) -> usize {
    let combinations: HashSet<String> = HashSet::from_iter(make_replacements(rules, start));
    combinations.len()
}


fn make_replacements(rules: &Rules, string: &str) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();
    let mut s_iter = string.chars().enumerate().peekable();
    while let Some((i, cur)) = s_iter.next() {
        // Single char
        if let Some(replacments) = rules.get(&cur.to_string()) {
            for repl in replacments {
                let new_s = format!("{}{}{}", &string[..i], repl, &string[i + 1..]);
                results.push(new_s);
            }
        }

        // Double char
        if let Some((n_i, next)) = s_iter.peek() {
            if let Some(replacments) = rules.get(&format!("{}{}", cur, next)) {
                for repl in replacments {
                    let new_s = format!("{}{}{}", &string[..i], repl, &string[n_i + 1..]);
                    results.push(new_s);
                }
                // Skip the next char since it was used already
                s_iter.next();
            }
        }
    }
    results
}

fn load_input(name: &str) -> (Rules, String) {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut rules: HashMap<String, Vec<String>> = HashMap::new();
    let mut lines = reader.lines().map(|l| l.unwrap());
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let (to, from) = line.split_once(" => ").unwrap();
        (*rules.entry(to.to_owned()).or_default()).push(from.to_owned());
    }
    (rules, lines.next().unwrap())
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    fn example_rules() -> Rules {
        let mut rules: Rules = HashMap::new();
        rules.insert("e".to_owned(), vec!["H".to_owned(), "O".to_owned()]);
        rules.insert("H".to_owned(), vec!["HO".to_owned(), "OH".to_owned()]);
        rules.insert("O".to_owned(), vec!["HH".to_owned()]);
        rules
    }

    #[test]
    fn part_1_example_1() {
        let rules = example_rules();
        assert_eq!(part_1(&rules, "HOH"), 4);
    }

    #[test]
    fn part_1_example_2() {
        let rules = example_rules();
        assert_eq!(part_1(&rules, "HOHOHO"), 7);
    }

    // #[test]
    // fn part_2_test() {
    //     let rules = example_rules();
    //     assert_eq!(part_2(&rules, "HOH"), 3);
    //     assert_eq!(part_2(&rules, "HOHOHO"), 7);
    // }
}
