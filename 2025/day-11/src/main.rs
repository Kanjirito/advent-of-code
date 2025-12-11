use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::ops::{Add, AddAssign};

use utils::BufReadExt;

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&input));
}

fn part_1(devices: &HashMap<String, Vec<String>>) -> usize {
    let mut path_counter: HashMap<String, usize> = HashMap::new();
    follow_connections("you", devices, &mut path_counter);
    *path_counter.get("you").unwrap()
}

fn part_2(devices: &HashMap<String, Vec<String>>) -> usize {
    let mut path_counter: HashMap<String, State> = HashMap::new();
    follow_connections_with_checks("svr", devices, &mut path_counter);
    path_counter.get("svr").unwrap().valid
}

fn follow_connections(
    cur: &str,
    devices: &HashMap<String, Vec<String>>,
    counter: &mut HashMap<String, usize>,
) -> usize {
    if let Some(n) = counter.get(cur) {
        return *n;
    }

    let mut cur_counter = 0;

    for next in &devices[cur] {
        if next == "out" {
            cur_counter += 1;
        } else {
            cur_counter += follow_connections(next, devices, counter);
        }
    }
    *counter.entry(cur.to_owned()).or_default() += cur_counter;
    cur_counter
}

fn follow_connections_with_checks<'a>(
    cur: &'a str,
    devices: &'a HashMap<String, Vec<String>>,
    counter: &mut HashMap<String, State>,
) -> State {
    if let Some(n) = counter.get(cur) {
        return *n;
    }

    let mut cur_state = State::default();
    let is_dac = cur == "dac";
    let is_fft = cur == "fft";
    for next in &devices[cur] {
        if next == "out" {
            if is_dac {
                cur_state.add_dac(1);
            } else if is_fft {
                cur_state.add_fft(1);
            } else {
                cur_state.add_none(1);
            }
        } else {
            let recurse = follow_connections_with_checks(next, devices, counter);

            if is_dac {
                cur_state.add_valid(recurse.fft + recurse.valid);
                cur_state.add_dac(recurse.dac + recurse.none);
            } else if is_fft {
                cur_state.add_valid(recurse.dac + recurse.valid);
                cur_state.add_fft(recurse.fft + recurse.none);
            } else {
                cur_state += recurse;
            }
        }
    }
    *counter.entry(cur.to_owned()).or_default() += cur_state;
    cur_state
}

fn load_input(name: &str) -> HashMap<String, Vec<String>> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut devices = HashMap::new();
    for line in reader.lines_unwrap() {
        let (device, outs) = line.split_once(": ").unwrap();
        let outputs: Vec<String> = outs.split(' ').map(|o| o.to_string()).collect();
        devices.insert(device.to_string(), outputs);
    }

    devices
}

#[derive(Debug, Default, Clone, Copy)]
struct State {
    none: usize,
    dac: usize,
    fft: usize,
    valid: usize,
}

impl Add for State {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.none += rhs.none;
        self.dac += rhs.dac;
        self.fft += rhs.fft;
        self.valid += rhs.valid;
        self
    }
}

impl AddAssign for State {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl State {
    fn add_none(&mut self, amount: usize) {
        self.none += amount;
    }

    fn add_dac(&mut self, amount: usize) {
        self.dac += amount;
    }

    fn add_fft(&mut self, amount: usize) {
        self.fft += amount;
    }

    fn add_valid(&mut self, amount: usize) {
        self.valid += amount;
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = load_input("example");
        assert_eq!(part_1(&input), 5);
    }

    #[test]
    fn part_2_test() {
        let input = load_input("example_2");
        assert_eq!(part_2(&input), 2);
    }
}
