use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let mut input = load_input();
    let mut first = input.pop().unwrap();
    while let Some(number) = input.pop() {
        add_other_number(&mut first, number);
        reduce_number(&mut first);
    }
    println!(
        "Solution for part 1: {}",
        get_magnitude(first.clone(), get_biggest_depth(&first))
    );
}

fn get_magnitude(mut number: Vec<Point>, mut lowest: usize) -> usize {
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
        for index in 0..number.len() {
            if number[index].depth >= 5 {
                let left = number[index].value;
                let right = number[index + 1].value;
                if index > 0 {
                    number[index - 1].value += left;
                }
                if index < number.len() - 2 {
                    number[index + 2].value += right;
                }
                let old = number.remove(index);
                number[index] = Point {
                    value: 0,
                    depth: old.depth - 1,
                };
                continue 'main;
            }
        }

        for index in 0..number.len() {
            let v = number[index].value;
            if v >= 10 {
                let old = number.remove(index);
                let div = v / 2;
                if v % 2 != 0 {
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
                            value: div + 1,
                            depth: old.depth + 1,
                        },
                    );
                } else {
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
                continue 'main;
            }
        }

        break;
    }
}

fn get_biggest_depth(input: &[Point]) -> usize {
    let mut counter = 0;
    for point in input {
        if point.depth > counter {
            counter = point.depth;
        }
    }
    counter
}

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
    input.reverse();
    input
}

#[derive(Clone)]
struct Point {
    value: usize,
    depth: usize,
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
