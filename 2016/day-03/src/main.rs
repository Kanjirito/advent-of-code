use std::fs::File;
use std::io::BufReader;

use utils::BufReadExt;

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&input));
}

fn part_1(triangles: &[[u64; 3]]) -> u64 {
    let mut counter = 0;
    for &(mut t) in triangles {
        t.sort();
        if check_triangle(&t) {
            counter += 1;
        }
    }
    counter
}

fn part_2(triangles: &[[u64; 3]]) -> u64 {
    let mut counter = 0;
    for t in triangles.chunks(3) {
        let mut first = [t[0][0], t[1][0], t[2][0]];
        let mut second = [t[0][1], t[1][1], t[2][1]];
        let mut third = [t[0][2], t[1][2], t[2][2]];
        first.sort();
        second.sort();
        third.sort();
        if check_triangle(&first) {
            counter += 1;
        }
        if check_triangle(&second) {
            counter += 1;
        }
        if check_triangle(&third) {
            counter += 1;
        }
    }
    counter
}

fn check_triangle(triangle: &[u64; 3]) -> bool {
    triangle[0] + triangle[1] > triangle[2] && triangle[0] + triangle[2] > triangle[1]
}

fn load_input(name: &str) -> Vec<[u64; 3]> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut input = vec![];
    for line in reader.lines_unwrap() {
        input.push([
            line[..5].trim().parse().unwrap(),
            line[5..10].trim().parse().unwrap(),
            line[10..].trim().parse().unwrap(),
        ]);
    }
    input
}
