use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Reindeer {
    speed: usize,
    dash_time: usize,
    rest_time: usize,
}

impl Reindeer {
    fn cycle(&self) -> usize {
        self.dash_time + self.rest_time
    }
}

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&input, 2503));
}

fn part_1(reindeers: &[Reindeer]) -> usize {
    reindeers
        .iter()
        .map(|r| calculate_distance(r, 2503))
        .max()
        .unwrap()
}

fn part_2(reindeers: &[Reindeer], count: usize) -> usize {
    // Vec< [distance, points] >
    let mut state = vec![[0, 0]; reindeers.len()];
    for turn in 0..count {
        let mut cur_lead = 0;

        // For each reindeer...
        for i in 0..reindeers.len() {
            let r = reindeers[i];
            // ...add the distance if the currently in the dash part of the cycle
            if turn % r.cycle() < r.dash_time {
                state[i][0] += r.speed;
            }
            cur_lead = cur_lead.max(state[i][0]);
        }

        // Give out points to everyone leading
        for [d, p] in state.iter_mut() {
            if *d == cur_lead {
                *p += 1;
            }
        }
    }
    state.iter().max_by_key(|[_, p]| p).unwrap()[1]
}

fn calculate_distance(reindeer: &Reindeer, time: usize) -> usize {
    let mut counter = 0;
    let (div, rem) = (time / reindeer.cycle(), time % reindeer.cycle());
    // For each full cycle add speed * dash_time
    counter += div * reindeer.speed * reindeer.dash_time;
    // If the reminder is smaller than dash_time use that
    counter += rem.min(reindeer.dash_time) * reindeer.speed;
    counter
}

fn load_input(name: &str) -> Vec<Reindeer> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let reg = Regex::new(r#".*?(\d+) km/s.*?(\d+) seconds.*?(\d+) seconds"#).unwrap();
    let mut list = Vec::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        let result = reg.captures(&line).unwrap();
        let speed = result.get(1).unwrap().as_str().parse().unwrap();
        let dash_time = result.get(2).unwrap().as_str().parse().unwrap();
        let rest_time = result.get(3).unwrap().as_str().parse().unwrap();
        list.push(Reindeer {
            speed,
            dash_time,
            rest_time,
        });
    }
    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comet_test_part_1() {
        let input = Reindeer {
            speed: 14,
            dash_time: 10,
            rest_time: 127,
        };
        assert_eq!(calculate_distance(&input, 1000), 1120);
    }

    #[test]
    fn dancer_test_part_1() {
        let input = Reindeer {
            speed: 16,
            dash_time: 11,
            rest_time: 162,
        };
        assert_eq!(calculate_distance(&input, 1000), 1056);
    }

    #[test]
    fn test_part_2() {
        let input = vec![
            Reindeer {
                speed: 14,
                dash_time: 10,
                rest_time: 127,
            },
            Reindeer {
                speed: 16,
                dash_time: 11,
                rest_time: 162,
            },
        ];
        assert_eq!(part_2(&input, 1000), 689);
    }

    // #[test]
    // fn part_2_test() {
    //     todo!()
    // }
}
