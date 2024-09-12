use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&input));

}

fn part_1(boxes: &[(usize, usize, usize)]) -> usize {
    let mut total = 0;
    for (x, y, z) in boxes {
        let one = x * y;
        let two = x * z;
        let three = y * z;
        total += 2 * (one + two + three);
        total += one.min(two).min(three);
    }
    total
}

fn part_2(boxes: &[(usize, usize, usize)]) -> usize {
    let mut total = 0;
    for b in boxes {
        let mut sorted = [b.0, b.1, b.2];
        sorted.sort();
        total += 2 * sorted[0] + 2 * sorted[1];
        total += sorted[0] * sorted[1] * sorted[2];
    }
    total
}

fn load_input(name: &str) -> Vec<(usize, usize, usize)> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut boxes: Vec<(usize, usize, usize)> = vec![];
    for line in reader
        .lines()
        .filter_map(|l| l.map_or(None, |l| if l.is_empty() { None } else { Some(l) }))
    {
        let split: Vec<usize> = line
            .split('x')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        boxes.push((split[0], split[1], split[2]))
    }
    boxes
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_1() {
        assert_eq!(super::part_1(&[(2, 3, 4)]), 58);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2(&[(2, 3, 4)]), 34);

    }
}
