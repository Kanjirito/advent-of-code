use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let numbers = load_input("input");
    println!("Part 1: {}", part_1(numbers.clone()));
    println!("Part 2: {}", part_2(numbers));
}

fn part_1(mut numbers: Vec<(usize, i64)>) -> i64 {
    shuffle_numbers(&mut numbers);
    get_result(&numbers)
}

fn part_2(mut numbers: Vec<(usize, i64)>) -> i64 {
    let key = 811589153;
    // Apply the key to the values
    numbers = numbers.into_iter().map(|(x, y)| (x, y * key)).collect();
    for _ in 0..10 {
        shuffle_numbers(&mut numbers);
    }
    get_result(&numbers)
}

fn get_result(numbers: &[(usize, i64)]) -> i64 {
    let (zero_i, _) = numbers
        .iter()
        .enumerate()
        .map(|x| (x.0, x.1 .1))
        .find(|(_, y)| *y == 0)
        .unwrap();

    let mut result = 0;
    for x in 1..=3 {
        let new_index = (zero_i + x * 1000) % numbers.len();
        result += numbers[new_index].1;
    }
    result
}

fn shuffle_numbers(numbers: &mut Vec<(usize, i64)>) {
    let l = (numbers.len() - 1) as i64;
    for counter in 0..numbers.len() {
        let mut index = 0;
        for (i, (x, _)) in numbers.iter().enumerate() {
            if *x == counter {
                index = i;
                break;
            }
        }
        let value = numbers.remove(index);
        // Calculates the target index.
        // Modulo is done because doing a full round doesn't change anything
        let mut new_pos = (index as i64 + value.1) % l;
        if new_pos < 0 {
            // If lower than zero then that's the index counting from the end which needs to be calculated
            new_pos = l - new_pos.abs();
        }
        numbers.insert(new_pos as usize, value);
    }
}

fn load_input(name: &str) -> Vec<(usize, i64)> {
    let file = File::open(name).expect("No input file found");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        .enumerate()
        .map(|(i, x)| (i, x))
        .collect()
}

#[test]
fn example() {
    let input = load_input("example");
    assert_eq!(part_1(input.clone()), 3);
    assert_eq!(part_2(input), 1623178306);
}
