use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input();

    // Part 1
    let gamma_binary = generate_gamma_binary(&input);
    let epsilon_binary = invert_binary(&gamma_binary);
    println!("Gamma binary:   {}", gamma_binary);
    println!("Epsilon binary: {}", epsilon_binary);
    let gamma_decimal = convert_to_decimal(&gamma_binary);
    let epsilon_decimal = convert_to_decimal(&epsilon_binary);
    println!("Solution to part 1: {}", gamma_decimal * epsilon_decimal);
    println!();

    // Part 2
    let generator_binary = filter_numbers(&input, '1');
    let scrubber_binary = filter_numbers(&input, '0');
    println!("Oxygen generator rating binary: {}", generator_binary);
    println!("C02 scrubber rating binary:     {}", scrubber_binary);
    let generator_decimal = convert_to_decimal(&generator_binary);
    let scrubber_decimal = convert_to_decimal(&scrubber_binary);
    println!(
        "Solution to part 2: {}",
        generator_decimal * scrubber_decimal
    );
}

fn generate_gamma_binary(input: &[String]) -> String {
    let mut counter: Vec<u64> = vec![0; input[0].len()];
    for line in input {
        for (index, digit) in line.chars().enumerate() {
            if digit == '1' {
                counter[index] += 1;
            };
        }
    }

    let mut gamma_string = String::new();
    let half = (input.len() / 2) as u64;
    for count in counter {
        if count > half {
            gamma_string.push('1');
        } else {
            gamma_string.push('0');
        }
    }
    gamma_string
}

fn invert_binary(input: &str) -> String {
    let mut inverted_binary = String::new();
    for digit in input.chars() {
        if digit == '1' {
            inverted_binary.push('0');
        } else {
            inverted_binary.push('1');
        }
    }
    inverted_binary
}

fn convert_to_decimal(input: &str) -> u64 {
    u64::from_str_radix(input, 2).expect("Wrong binary")
}

fn filter_numbers(input: &[String], looking_for: char) -> String {
    let mut index: usize = 0;
    let mut filtered_vec: Vec<String> = input.to_vec();
    loop {
        let most_common: SearchResult = find_most_common(&filtered_vec, index);
        let wanted_char = if looking_for == '1' {
            match most_common {
                SearchResult::Zero => '0',
                SearchResult::One | SearchResult::Draw => '1',
            }
        } else {
            match most_common {
                SearchResult::Zero => '1',
                SearchResult::One | SearchResult::Draw => '0',
            }
        };
        filtered_vec = filtered_vec
            .into_iter()
            .filter(|value| {
                let digit = value.chars().nth(index).unwrap();
                digit == wanted_char
            })
            .collect();
        if filtered_vec.len() == 1 {
            return filtered_vec.pop().unwrap();
        } else {
            index += 1;
        }
    }
}

enum SearchResult {
    Zero,
    One,
    Draw,
}

#[allow(clippy::comparison_chain)]
fn find_most_common(input: &[String], index: usize) -> SearchResult {
    let half = input.len() / 2;
    let mut counter: usize = 0;
    for line in input {
        if line.chars().nth(index).unwrap() == '1' {
            counter += 1;
        };
    }

    if counter > half {
        SearchResult::One
    } else if counter < half {
        SearchResult::Zero
    } else {
        SearchResult::Draw
    }
}

fn load_input() -> Vec<String> {
    let file = File::open("input").expect("No input file found");
    let reader = BufReader::new(file);
    let loaded_file: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Invalid line"))
        .collect();
    loaded_file
}
