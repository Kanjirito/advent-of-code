use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input();

    // Part 1
    let mut counter: usize = 0;
    // Because who needs readable loops?
    for lenght in input.iter().flat_map(|x| &x.1).map(|x| x.len()) {
        match lenght {
            2 | 3 | 4 | 7 => counter += 1,
            _ => {}
        }
    }
    println!("Solution for part 1: {}", counter);

    // Part 2
    let mut counter: usize = 0;
    for line in input {
        let mut one: String = String::new();
        let mut four: String = String::new();
        // let mut seven: String = String::new();
        // let mut eight: String = String::new();
        for sequence in line.0.iter().chain(line.1.iter()) {
            match sequence.len() {
                2 => one = sequence.to_string(),
                // 3 => seven = sequence.to_string(),
                4 => four = sequence.to_string(),
                // 7 => eight = sequence.to_string(),
                _ => {}
            }
        }

        let mut output: String = String::new();
        for out in line.1 {
            match out.len() {
                2 => output.push('1'),
                3 => output.push('7'),
                4 => output.push('4'),
                5 => {
                    // if all of the "1" characters are in out
                    if one.chars().all(|x| out.contains(x)) {
                        output.push('3');
                    // if 3 of the "4" characters are in out
                    } else if four.chars().filter(|x| out.contains(*x)).count() == 3 {
                        output.push('5');
                    } else {
                        output.push('2');
                    }
                }
                6 => {
                    // if all of the "4" characters are in out
                    if four.chars().all(|x| out.contains(x)) {
                        output.push('9');
                    // if all of the "1" characters are in out
                    } else if one.chars().all(|x| out.contains(x)) {
                        output.push('0');
                    } else {
                        output.push('6');
                    }
                }
                7 => output.push('8'),
                _ => {}
            }
        }
        counter += output.parse::<usize>().unwrap();
    }
    println!("Solution for part 2: {}", counter);
}

fn load_input() -> Vec<(Vec<String>, Vec<String>)> {
    let file = File::open("input").expect("No input file found");
    let reader = BufReader::new(file);
    let mut codes: Vec<(Vec<String>, Vec<String>)> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split(" | ");
        let first: Vec<String> = split
            .next()
            .unwrap()
            .split(' ')
            .map(|x| x.to_string())
            .collect();
        let second: Vec<String> = split
            .next()
            .unwrap()
            .split(' ')
            .map(|x| x.to_string())
            .collect();
        codes.push((first, second));
    }
    codes
}
