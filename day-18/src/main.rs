use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input();
    let mut first = input[0].clone();
    for other_number in input[1..].iter().cloned() {
        add_other_number(&mut first, other_number);
        reduce_number(&mut first);
    }
    println!("Solution for part 1: {}", get_magnitude(first.clone()));

    let mut highest_magnitude = 0;
    for first in input.iter().cloned() {
        for second in input.iter().cloned() {
            if first == second {
                continue;
            }
            let mut current_first = first.clone();
            add_other_number(&mut current_first, second);
            reduce_number(&mut current_first);
            let mag = get_magnitude(current_first);
            if mag > highest_magnitude {
                highest_magnitude = mag;
            }
        }
    }
    println!("Solution for part 2: {}", highest_magnitude);
}

/// Gets the magnitude of the number
///
/// First gets the biggest depth then looks for numbers at that depth. Since pairs at the biggest depths are always regular numbers
/// the pairs will always be (index, index + 1). Each pair at that depth gets replaced with a new number at a lower depth with the value
/// equal to it's magnitude. Once there are no more numbers at the lowest depth, decrement it and do it again. When the biggest depth is 1 it
/// means there are only 2 numbers left so just calculate their magnitude and return it.
fn get_magnitude(mut number: Vec<Point>) -> usize {
    let mut lowest = get_biggest_depth(&number);
    'main: loop {
        if lowest == 1 {
            return (number[0].value * 3) + (number[1].value * 2);
        }
        for index in 0..number.len() {
            if number[index].depth == lowest {
                let new_num = (number[index].value * 3) + (number[index + 1].value * 2);
                number.remove(index);
                number[index] = Point {
                    value: new_num,
                    depth: lowest - 1,
                };
                continue 'main;
            }
        }
        lowest -= 1;
    }
}

fn reduce_number(number: &mut Vec<Point>) {
    'main: loop {
        // Explode search
        // Find the first number that can be exploded
        for index in 0..number.len() {
            if number[index].depth >= 5 {
                let left = number[index].value;
                let right = number[index + 1].value;

                // Explode to left if possible
                if index > 0 {
                    number[index - 1].value += left;
                }
                // Explode to right if possible
                if index < number.len() - 2 {
                    number[index + 2].value += right;
                }

                // Remove first of pair which moves to next to it's index
                let old = number.remove(index);
                // Change second in pair to value 0 with a lower depth
                number[index] = Point {
                    value: 0,
                    depth: old.depth - 1,
                };
                // If exploded restart the loop to look for more to explode
                continue 'main;
            }
        }

        // Split search
        // Find first number that can be split
        // Will only get reached if nothing exploded
        for index in 0..number.len() {
            let v = number[index].value;
            if v >= 10 {
                // Gets the number to split
                let old = number.remove(index);
                let div = v / 2;

                // If not divisible by 2
                if v % 2 != 0 {
                    // First of the new pair, rounded down
                    number.insert(
                        index,
                        Point {
                            value: div,
                            depth: old.depth + 1,
                        },
                    );
                    // Second of the new pair, rounded up
                    number.insert(
                        index + 1,
                        Point {
                            value: div + 1,
                            depth: old.depth + 1,
                        },
                    );
                } else {
                    // Divisible by 2
                    number.insert(
                        index,
                        Point {
                            value: div,
                            depth: old.depth + 1,
                        },
                    );
                    number.insert(
                        index + 1,
                        Point {
                            value: div,
                            depth: old.depth + 1,
                        },
                    );
                }
                // If split happened loop again from start
                continue 'main;
            }
        }
        // Will only get reached if no explosion or split happen meaning the number is reduced
        break;
    }
}

/// Iterates through points and returns the biggest depth found
fn get_biggest_depth(input: &[Point]) -> usize {
    let mut counter = 0;
    for point in input {
        if point.depth > counter {
            counter = point.depth;
        }
    }
    counter
}

/// Adds 2 numbers together
fn add_other_number(first: &mut Vec<Point>, mut second: Vec<Point>) {
    first.append(&mut second);
    for point in first {
        point.depth += 1;
    }
}

fn load_input() -> Vec<Vec<Point>> {
    let file = File::open("input").expect("No input file found");
    let reader = BufReader::new(file);
    let mut input = Vec::new();
    for line in reader.lines() {
        let mut arena = Vec::new();
        let mut depth_counter = 0;
        for c in line.unwrap().chars() {
            match c {
                '[' => depth_counter += 1,
                ']' => depth_counter -= 1,
                ',' => {}
                other => arena.push(Point {
                    depth: depth_counter,
                    value: other.to_digit(10).unwrap() as usize,
                }),
            }
        }
        input.push(arena);
    }
    input
}

#[derive(Clone, PartialEq, Eq)]
struct Point {
    value: usize,
    depth: usize,
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
