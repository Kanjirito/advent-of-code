use std::fs::File;
use std::io::BufReader;

use utils::BufReadExt;

fn main() {
    let (ranges, ids) = load_input("input");
    println!("Solution for part 1: {}", part_1(&ranges, &ids));
    println!("Solution for part 2: {}", part_2(&ranges));
}

fn part_1(ranges: &[(u64, u64)], ids: &[u64]) -> u64 {
    let mut counter = 0;

    'main: for id in ids {
        for (start, end) in ranges {
            if (start..=end).contains(&id) {
                counter += 1;
                continue 'main;
            }
        }
    }

    counter
}

fn part_2(ranges: &[(u64, u64)]) -> u64 {
    let mut counter = 0;

    let combined = combine_ranges(ranges);

    for (start, end) in combined {
        counter += end - start + 1;
    }

    counter
}

fn combine_ranges(ranges: &[(u64, u64)]) -> Vec<(u64, u64)> {
    let mut combined = vec![ranges[0]];

    for &(start, end) in ranges.iter().skip(1) {
        let last = *combined.last().unwrap();
        if start > last.1 {
            combined.push((start, end));
        } else {
            combined.pop();
            combined.push((last.0, end.max(last.1)));
        }
    }

    combined
}

fn load_input(name: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut lines = reader.lines_unwrap();

    let mut ranges = vec![];

    for line in &mut lines {
        if line.is_empty() {
            break;
        }

        let (start, end) = line.split_once('-').unwrap();
        ranges.push((start.parse().unwrap(), end.parse().unwrap()));
    }

    ranges.sort_unstable();

    let ids = lines.map(|l| l.parse().unwrap()).collect();

    (ranges, ids)
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let (ranges, ids) = load_input("example");
        assert_eq!(part_1(&ranges, &ids), 3);
    }

    #[test]
    fn part_2_test() {
        let (ranges, _ids) = load_input("example");
        assert_eq!(part_2(&ranges), 14);
    }
}
