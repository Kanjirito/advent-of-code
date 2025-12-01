use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&input));
}

fn part_1(rotations: &[i64]) -> u64 {
    let mut counter = 0;
    let mut cur_value = 50;
    for rot in rotations {
        cur_value += rot;
        cur_value = cur_value.rem_euclid(100);
        if cur_value == 0 {
            counter += 1;
        }
    }
    counter
}

fn part_2(rotations: &[i64]) -> i64 {
    let mut counter = 0;
    let mut cur_value: i64 = 50;
    for rot in rotations {
        for _ in 0..rot.abs() {
            if *rot >= 0 {
                cur_value += 1;
            } else {
                cur_value -= 1;
            }

            cur_value = cur_value.rem_euclid(100);
            if cur_value == 0 {
                counter += 1;
            }
        }
    }
    counter
}

fn load_input(name: &str) -> Vec<i64> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap());
    let mut rotations = vec![];
    for l in lines {
        let (dire, num) = l.split_at(1);
        let num: i64 = num.parse().unwrap();
        if dire == "L" {
            rotations.push(-num);
        } else {
            rotations.push(num);
        }
    }
    rotations
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = load_input("example");
        assert_eq!(part_1(&input), 3);
    }

    #[test]
    fn part_2_test() {
        let input = load_input("example");
        assert_eq!(part_2(&input), 6);
    }
}
