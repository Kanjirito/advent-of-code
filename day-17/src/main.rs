#![allow(unused_variables, dead_code)]
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", solve(&input, 150, false));
    println!("Solution for part 2: {}", solve(&input, 150, true));
}

fn solve(containers: &[usize], target: usize, part_2: bool) -> usize {
    let mut lowest = usize::MAX;
    let mut counter = 0;
    let mut q: Vec<(Vec<usize>, Vec<usize>)> =
        vec![(Vec::with_capacity(containers.len()), containers.to_vec())];
    while let Some(mut cur) = q.pop() {
        if part_2 && cur.0.len() > lowest {
            continue;
        }
        match target.cmp(&cur.0.iter().sum::<usize>()) {
            std::cmp::Ordering::Greater => (),
            std::cmp::Ordering::Equal => {
                if part_2 {
                    match cur.0.len().cmp(&lowest) {
                        std::cmp::Ordering::Less => {
                            counter = 1;
                            lowest = cur.0.len();
                        }
                        std::cmp::Ordering::Equal => {
                            counter += 1;
                        }
                        std::cmp::Ordering::Greater => unreachable!(),
                    }
                } else {
                    counter += 1;
                }
                continue;
            }
            std::cmp::Ordering::Less => continue,
        }

        while let Some(other) = cur.1.pop() {
            let mut new_selected = cur.0.clone();
            let new_left = cur.1.clone();
            new_selected.push(other);
            q.push((new_selected, new_left));
        }
    }
    counter
}
fn load_input(name: &str) -> Vec<usize> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut containers: Vec<_> = reader
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();
    containers.sort_unstable();
    containers
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = vec![5, 5, 10, 15, 20];
        assert_eq!(solve(&input, 25, false), 4);
    }

    #[test]
    fn part_2_test() {
        let input = vec![5, 5, 10, 15, 20];
        assert_eq!(solve(&input, 25, true), 3);
    }
}
