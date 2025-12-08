use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

type Box = (usize, usize, usize);

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(input.clone(), 1000));
    println!("Solution for part 2: {}", part_2(input));
}

fn part_1(mut boxes: HashMap<Box, Vec<Box>>, count: usize) -> usize {
    let mut group_sizes = vec![];
    let distances = get_all_distances(&boxes);

    for &(_, first, second) in &distances[..count] {
        boxes.get_mut(&first).unwrap().push(second);
        boxes.get_mut(&second).unwrap().push(first);
    }
    let mut seen: HashSet<Box> = HashSet::new();

    for cur in boxes.keys() {
        group_sizes.push(visit_connections(*cur, &boxes, &mut seen));
    }
    group_sizes.sort();
    group_sizes.iter().rev().take(3).product()
}

fn part_2(mut boxes: HashMap<Box, Vec<Box>>) -> usize {
    let distances = get_all_distances(&boxes);

    for &(_, first, second) in &distances {
        boxes.get_mut(&first).unwrap().push(second);
        boxes.get_mut(&second).unwrap().push(first);
        let mut seen: HashSet<Box> = HashSet::new();
        let size = visit_connections(*boxes.keys().next().unwrap(), &boxes, &mut seen);
        if size == boxes.len() {
            return first.0 * second.0;
        }
    }
    panic!("shouldn't be here")
}

fn visit_connections(cur: Box, boxes: &HashMap<Box, Vec<Box>>, seen: &mut HashSet<Box>) -> usize {
    if seen.contains(&cur) {
        return 0;
    }
    seen.insert(cur);
    let mut counter = 1;

    for &next in &boxes[&cur] {
        counter += visit_connections(next, boxes, seen);
    }

    counter
}

fn get_all_distances(boxes: &HashMap<Box, Vec<Box>>) -> Vec<(f64, Box, Box)> {
    let mut all_connections = vec![];
    let keys: Vec<Box> = boxes.keys().cloned().collect();

    for (i, &cur) in keys[..keys.len() - 1].iter().enumerate() {
        for &other in &keys[i + 1..] {
            let distance = get_distance(cur, other);
            if distance != 0.0 {
                all_connections.push((distance, cur, other));
            }
        }
    }

    all_connections.sort_unstable_by(|a, b| a.0.total_cmp(&b.0));
    all_connections
}

fn load_input(name: &str) -> HashMap<Box, Vec<Box>> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| (new_box(&l.unwrap()), Vec::new()))
        .collect()
}

fn new_box(s: &str) -> Box {
    let mut splits = s.splitn(3, ',');
    let x = splits.next().unwrap().parse().unwrap();
    let y = splits.next().unwrap().parse().unwrap();
    let z = splits.next().unwrap().parse().unwrap();
    (x, y, z)
}

fn get_distance(first: Box, other: Box) -> f64 {
    let x_1 = first.0 as f64;
    let y_1 = first.1 as f64;
    let z_1 = first.2 as f64;

    let x_2 = other.0 as f64;
    let y_2 = other.1 as f64;
    let z_2 = other.2 as f64;
    ((x_1 - x_2).powi(2) + (y_1 - y_2).powi(2) + (z_1 - z_2).powi(2)).sqrt()
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = load_input("example");
        assert_eq!(part_1(input, 10), 40);
    }

    #[test]
    fn part_2_test() {
        let input = load_input("example");
        assert_eq!(part_2(input), 25272);
    }
}
