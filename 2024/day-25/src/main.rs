use std::fs::File;
use std::io::BufReader;

use utils::BufReadExt;

type Key = [usize; 5];
type Locks = [[[[[usize; 6]; 6]; 6]; 6]; 6];

fn main() {
    let (keys, locks) = load_input("input");
    println!("Solution for part 1: {}", part_1(&keys, &locks));
}

#[allow(clippy::needless_range_loop)]
fn part_1(keys: &[Key], locks: &Locks) -> usize {
    let mut counter = 0;

    for key in keys {
        for first in 0..=(5 - key[0]) {
            for second in 0..=(5 - key[1]) {
                for third in 0..=(5 - key[2]) {
                    for fourth in 0..=(5 - key[3]) {
                        for fifth in 0..=(5 - key[4]) {
                            counter += locks[first][second][third][fourth][fifth];
                        }
                    }
                }
            }
        }
    }

    counter
}

fn load_input(name: &str) -> (Vec<Key>, Locks) {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut keys = Vec::new();
    let mut locks = [[[[[0; 6]; 6]; 6]; 6]; 6];
    let mut lines = reader.lines_unwrap();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        } else if line == "#####" {
            // Key
            let mut heights = [0; 5];
            for _ in 0..5 {
                for (i, x) in lines.next().unwrap().chars().enumerate() {
                    if x == '#' {
                        heights[i] += 1;
                    }
                }
            }
            keys.push(heights);
        } else {
            // Lock
            let mut heights = [5; 5];
            for _ in 0..5 {
                for (i, x) in lines.next().unwrap().chars().enumerate() {
                    if x == '.' {
                        heights[i] -= 1;
                    }
                }
            }
            locks[heights[0]][heights[1]][heights[2]][heights[3]][heights[4]] += 1;
        }
        lines.next();
        lines.next();
    }

    (keys, locks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let (keys, locks) = load_input("example");
        assert_eq!(keys, [[0, 5, 3, 4, 3], [1, 2, 0, 5, 3]]);
        assert_eq!(locks[5][0][2][1][3], 1);
        assert_eq!(locks[4][3][4][0][2], 1);
        assert_eq!(locks[3][0][2][0][1], 1);
    }
}
