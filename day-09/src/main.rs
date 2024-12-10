use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const NEIGHBOURS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

fn main() {
    let input = load_input();

    // Part 1
    let mut counter: usize = 0;
    let mut low_points: Vec<(usize, usize)> = Vec::new();
    for (x, column) in input.iter().enumerate() {
        'number: for (y, number) in column.iter().enumerate() {
            if number == &9 {
                continue;
            }
            for (x_offset, y_offset) in NEIGHBOURS {
                if number
                    >= &input[(x as isize + x_offset) as usize][(y as isize + y_offset) as usize]
                {
                    continue 'number;
                }
            }

            low_points.push((x, y));
            counter += 1 + number;
        }
    }
    println!("Solution for part 1: {}", counter);

    // Part 2 (Recursive)
    let mut counters: Vec<usize> = Vec::new();
    for cords in &low_points {
        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        flood_fill_recursive(*cords, &input, &mut seen);
        counters.push(seen.len());
    }
    counters.sort_unstable();
    let mut counter: usize = 1;
    for c in counters.iter().rev().take(3) {
        counter *= c;
    }
    println!("Solution for part 2 (recursive): {}", counter);

    // Part 2 (Queue)
    counters = Vec::new();
    for cords in &low_points {
        counters.push(flood_fill_deque(*cords, &input));
    }
    counters.sort_unstable();
    counter = 1;
    for c in counters.iter().rev().take(3) {
        counter *= c;
    }
    println!("Solution for part 2 (queue):     {}", counter);
}

fn flood_fill_recursive(
    cords: (usize, usize),
    input: &[Vec<usize>],
    seen: &mut HashSet<(usize, usize)>,
) {
    let x = cords.0;
    let y = cords.1;
    seen.insert(cords);
    for (x_offset, y_offset) in NEIGHBOURS {
        let new_x = (x as isize + x_offset) as usize;
        let new_y = (y as isize + y_offset) as usize;
        let new_cords = (new_x, new_y);

        if input[new_x][new_y] != 9 && !seen.contains(&new_cords) {
            flood_fill_recursive(new_cords, input, seen);
        }
    }
}

fn flood_fill_deque(start_cords: (usize, usize), input: &[Vec<usize>]) -> usize {
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back(start_cords);
    while !queue.is_empty() {
        let cords = queue.pop_front().unwrap();
        seen.insert(cords);
        for (x_offset, y_offset) in NEIGHBOURS {
            let new_x = (cords.0 as isize + x_offset) as usize;
            let new_y = (cords.1 as isize + y_offset) as usize;
            let new_cords = (new_x, new_y);
            if input[new_x][new_y] != 9 && !seen.contains(&new_cords) {
                queue.push_back(new_cords)
            }
        }
    }
    seen.len()
}

fn load_input() -> Vec<Vec<usize>> {
    let file = File::open("input").expect("No input file found");
    let reader = BufReader::new(file);
    let mut input: Vec<Vec<usize>> = Vec::new();
    for line in reader.lines() {
        let line: String = format!("{}{}{}", 9, line.unwrap(), 9);
        input.push(line.split("").flat_map(|x| x.parse()).collect());
    }
    let l = input[0].len();
    input.insert(0, vec![9; l]);
    input.push(vec![9; l]);
    input
}
