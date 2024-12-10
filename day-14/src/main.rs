use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Position = (u64, u64);

fn main() {
    let (input, highest) = load_input("input");
    let mut map = map_rocks(&input);
    let (one, two) = solve(highest, &mut map);
    println!("Part 1: {}", one);
    println!("Part 2: {}", two);
}

fn solve(highest: u64, map: &mut HashSet<Position>) -> (u64, u64) {
    // Counts how many sand units are rested
    let mut counter = 0;
    let mut part_1_counter: Option<u64> = None;
    'outer: loop {
        let mut cur_pos: Position = (500, 0);
        // Check for part 2, if true then everything is taken
        while !map.contains(&cur_pos) {
            // Check for part 1, if `pos.1` is equal the highest value then the sand is in the "abyss"
            if cur_pos.1 == highest && part_1_counter.is_none() {
                part_1_counter = Some(counter);
            // Simulates the floor. If 1 above it then it can't get any lower
            } else if cur_pos.1 == highest + 1 {
                map.insert(cur_pos);
                counter += 1;
                continue 'outer;
            }

            // Checks possible positions, if free move there and check again
            let down = (cur_pos.0, cur_pos.1 + 1);
            if !map.contains(&down) {
                cur_pos = down;
                continue;
            }
            let left_down = (cur_pos.0 - 1, cur_pos.1 + 1);
            if !map.contains(&left_down) {
                cur_pos = left_down;
                continue;
            }
            let left_right = (cur_pos.0 + 1, cur_pos.1 + 1);
            if !map.contains(&left_right) {
                cur_pos = left_right;
                continue;
            }
            // No position was free meaning the sand unit will rest here
            // Marks the position as taken, increases the counter and starts from the top again
            map.insert(cur_pos);
            counter += 1;
            continue 'outer;
        }
        break;
    }
    (part_1_counter.unwrap(), counter)
}

fn map_rocks(input: &[Vec<Position>]) -> HashSet<Position> {
    let mut map = HashSet::new();
    for formation in input {
        for pair in formation.windows(2) {
            let (x, y) = pair[0];
            let (i, j) = pair[1];
            if x == i {
                for new_y in y.min(j)..=y.max(j) {
                    map.insert((x, new_y));
                }
            } else if y == j {
                for new_x in x.min(i)..=x.max(i) {
                    map.insert((new_x, y));
                }
            }
        }
    }
    map
}

fn load_input(name: &str) -> (Vec<Vec<Position>>, u64) {
    let file = File::open(name).expect("No input file found");
    let reader = BufReader::new(file);
    let mut highest = 0;
    let mut input: Vec<Vec<Position>> = Vec::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        let mut rock_formation = Vec::new();
        for pos in line.split(" -> ") {
            let mut nums = pos.split(',');
            let x: u64 = nums.next().unwrap().parse().unwrap();
            let y: u64 = nums.next().unwrap().parse().unwrap();
            highest = highest.max(y);
            rock_formation.push((x, y));
        }
        input.push(rock_formation);
    }
    (input, highest)
}

#[test]
fn example() {
    let (input, highest) = load_input("example");
    let mut map = map_rocks(&input);
    assert_eq!(solve(highest, &mut map), (24, 93));
}
