use std::fs::File;
use std::io::BufReader;

use utils::BufReadExt;

/// (x, y)
type Red = (usize, usize);

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    // println!("Solution for part 2: {}", part_2(&input));
}

fn part_1(reds: &[Red]) -> usize {
    let mut biggest_area: usize = 0;

    for cur in reds {
        for other in reds {
            if cur == other {
                continue;
            }

            let x_diff = cur.0.abs_diff(other.0) + 1;
            let y_diff = cur.1.abs_diff(other.1) + 1;
            let area = x_diff * y_diff;
            if area > biggest_area {
                biggest_area = area;
            }
        }
    }

    biggest_area
}

fn load_input(name: &str) -> Vec<Red> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    reader
        .lines_unwrap()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = load_input("example");
        assert_eq!(part_1(&input), 50);
    }

    // #[test]
    // fn part_2_test() {
    //     let input = load_input("example");
    //     assert_eq!(part_2(&input), 24);
    // }
}
