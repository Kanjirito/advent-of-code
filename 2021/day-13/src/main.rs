use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

fn main() {
    let (mut points, instructions) = load_input();
    let mut first_run = true;
    for instruction in instructions.iter() {
        points = fold_paper(instruction, &points);
        if first_run {
            println!("Solution for part 1: {}", points.len());
            first_run = false;
        }
    }
    print_output(&points);
}

fn fold_paper(instruction: &Split, points: &HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
    let mut new_points: HashSet<(usize, usize)> = HashSet::new();
    match instruction {
        Split::Vertical(split_x) => {
            for point in points {
                if point.0 > *split_x {
                    let difference = point.0 - split_x;
                    if difference <= *split_x {
                        new_points.insert((split_x - difference, point.1));
                    }
                } else {
                    new_points.insert(*point);
                }
            }
        }
        Split::Horizontal(split_y) => {
            for point in points {
                if point.1 > *split_y {
                    let difference = point.1 - split_y;
                    if difference <= *split_y {
                        new_points.insert((point.0, split_y - difference));
                    }
                } else {
                    new_points.insert(*point);
                }
            }
        }
    };
    new_points
}

fn print_output(points: &HashSet<(usize, usize)>) {
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    println!("Solution for part 2:\n");
    for point in points {
        if point.0 > max_x {
            max_x = point.0;
        }
        if point.1 > max_y {
            max_y = point.1;
        }
    }
    for y in 0..=max_y {
        let mut line = String::new();
        for x in 0..=max_x {
            if points.contains(&(x, y)) {
                line.push('X');
            } else {
                line.push(' ')
            }
        }
        println!("{}", line);
    }
}

fn load_input() -> (HashSet<(usize, usize)>, Vec<Split>) {
    let file = File::open("input").expect("No input file found");
    let reader = BufReader::new(file);
    let mut points: HashSet<(usize, usize)> = HashSet::new();
    let mut instructions: Vec<Split> = Vec::new();
    for line in reader.lines() {
        let text = line.unwrap();
        if text.is_empty() {
            continue;
        }
        if let Ok(split) = Split::from_str(&text) {
            instructions.push(split);
        } else {
            let nums: Vec<usize> = text.split(',').flat_map(|x| x.parse::<usize>()).collect();
            points.insert((nums[0], nums[1]));
        }
    }
    (points, instructions)
}

#[derive(Debug, Clone, Copy)]
enum Split {
    Vertical(usize),
    Horizontal(usize),
}

impl FromStr for Split {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("fold along") {
            Err(())
        } else {
            let mut chars = s[11..].split('=').rev();
            let num: usize = chars.next().unwrap().parse().unwrap();
            Ok(match chars.next().unwrap() {
                "y" => Self::Horizontal(num),
                "x" => Self::Vertical(num),
                _ => return Err(()),
            })
        }
    }
}
