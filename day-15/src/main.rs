use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const SIZE: i64 = 4000000;

fn main() {
    let input = load_input("input");
    println!("Part 1: {}", part_1(&input, 2000000));
    println!("Part 2: {}", part_2(&input, SIZE));
}

fn part_1(input: &[Sensor], to_check: i64) -> usize {
    // Keeps track of the taken spots
    let mut taken: HashSet<i64> = HashSet::new();
    for sensor in input {
        let height_diff = sensor.y.abs_diff(to_check) as i64;
        if height_diff <= sensor.distance {
            let start = sensor.x - (sensor.distance - height_diff);
            let end = sensor.x + (sensor.distance - height_diff);
            for new_x in start..=end {
                taken.insert(new_x);
            }
            // The beacon needs to be removed if it's on the same line since it's a valid spot
            if sensor.beacon.1 == to_check {
                taken.remove(&sensor.beacon.0);
            }
        }
    }
    taken.len()
}

fn part_2(sensors: &[Sensor], limit: i64) -> i64 {
    for row in 0..=limit {
        // Create a list of the ranges that are covered on the given row by the sensors
        let mut ranges: Vec<(i64, i64)> = Vec::new();
        for sensor in sensors {
            let height_diff = sensor.y.abs_diff(row) as i64;
            if height_diff <= sensor.distance {
                let start = sensor.x - (sensor.distance - height_diff);
                let end = sensor.x + (sensor.distance - height_diff);
                ranges.push((start, end));
            }
        }
        // Sort them by the start value
        ranges.sort_unstable_by_key(|r| r.0);
        let mut cur_range = ranges[0];
        // Iterate over the ranges
        for range in ranges.iter().skip(1) {
            // If the end of the current range is over the current search limit stop checking the rest and continue to next row
            if cur_range.1 > limit {
                break;
            }
            // Start of the next range is before the end of cur range meaning they overlap
            if range.0 <= cur_range.1 {
                if range.1 <= cur_range.1 {
                    // End of the next range is before the end of the cur range meaning they overlap completely and there is no need to do anything
                    continue;
                } else {
                    // End of the next range is after the end of the cur range so the end gets moves
                    cur_range.1 = range.1;
                }
            } else {
                // Start of the next range leaves a gap meaning the free spot is end of cur range + 1
                return (cur_range.1 + 1) * SIZE + row;
            }
        }
    }
    unreachable!()
}

fn load_input(name: &str) -> Vec<Sensor> {
    let file = File::open(name).expect("No input file found");
    let reader = BufReader::new(file);
    let re = Regex::new(r".*?(-?\d+).*?(-?\d+).*?(-?\d+).*?(-?\d+)").unwrap();
    let mut sensors: Vec<Sensor> = Vec::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        let nums: Vec<i64> = re
            .captures(&line)
            .unwrap()
            .iter()
            .skip(1)
            .map(|m| m.unwrap().as_str().parse().unwrap())
            .collect();
        let distance: i64 = (nums[0].abs_diff(nums[2]) + nums[1].abs_diff(nums[3])) as i64;
        let sensor = Sensor {
            x: nums[0],
            y: nums[1],
            distance,
            beacon: (nums[2], nums[3]),
        };
        sensors.push(sensor);
    }
    sensors
}

#[derive(Debug)]
struct Sensor {
    x: i64,
    y: i64,
    distance: i64,
    beacon: (i64, i64),
}

#[test]
fn example() {
    let input = load_input("example");
    assert_eq!(part_1(&input, 10), 26);
    assert_eq!(part_2(&input, 20), 56000011);
}
