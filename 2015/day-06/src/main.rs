#![allow(unused_variables, dead_code)]
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct Instruction {
    first: (usize, usize),
    second: (usize, usize),
    action: Action,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let reg = Regex::new(r"(\d*),(\d*) through (\d*),(\d*)").unwrap();

        let action = if value.starts_with("turn on") {
            Action::On
        } else if value.starts_with("turn off") {
            Action::Off
        } else {
            Action::Switch
        };
        let caps = reg.captures(value).unwrap();
        let mut caps_iter = caps
            .iter()
            .skip(1)
            .map(|c| c.unwrap().as_str().parse::<usize>().unwrap());

        Instruction {
            first: (caps_iter.next().unwrap(), caps_iter.next().unwrap()),
            second: (caps_iter.next().unwrap(), caps_iter.next().unwrap()),
            action,
        }
    }
}

#[derive(Debug)]
enum Action {
    On,
    Off,
    Switch,
}

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&input));
}

fn part_1(ins: &[Instruction]) -> usize {
    let mut lights = vec![vec![false; 1000]; 1000];
    for i in ins {
        for row in lights.iter_mut().take(i.second.0 + 1).skip(i.first.0) {
            for light in row.iter_mut().take(i.second.1 + 1).skip(i.first.1) {
                match i.action {
                    Action::On => *light = true,
                    Action::Off => *light = false,
                    Action::Switch => *light = !*light,
                }
            }
        }
    }
    lights
        .into_iter()
        .flat_map(|x| x.into_iter())
        .filter(|x| *x)
        .count()
}

fn part_2(ins: &[Instruction]) -> usize {
    let mut lights = vec![vec![0_usize; 1000]; 1000];
    for i in ins {
        for row in lights.iter_mut().take(i.second.0 + 1).skip(i.first.0) {
            for light in row.iter_mut().take(i.second.1 + 1).skip(i.first.1) {
                match i.action {
                    Action::On => *light += 1,
                    Action::Off => *light = light.saturating_sub(1),
                    Action::Switch => *light += 2,
                }
            }
        }
    }
    lights.into_iter().flat_map(|x| x.into_iter()).sum()
}

fn load_input(name: &str) -> Vec<Instruction> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut instructions = Vec::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        instructions.push(line.as_str().into());
    }
    instructions
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example_1() {
        let ins = vec![Instruction::from("turn on 0,0 through 999,999")];
        assert_eq!(part_1(&ins), 1000000);
    }

    #[test]
    fn part_1_example_2() {
        let ins = vec![Instruction::from("toggle 0,0 through 999,0")];
        assert_eq!(part_1(&ins), 1000);
    }

    #[test]
    fn part_1_example_3() {
        let ins = vec![Instruction::from("turn on 499,499 through 500,500")];
        assert_eq!(part_1(&ins), 4);
    }

    #[test]
    fn part_1_turn_on_and_off() {
        let ins = vec![
            Instruction::from("turn on 499,499 through 500,500"),
            Instruction::from("turn off 499,499 through 500,500"),
        ];
        assert_eq!(part_1(&ins), 0);
    }

    #[test]
    fn part_1_turn_on_all() {
        let ins = vec![Instruction::from("turn on 0,0 through 999,999")];
        assert_eq!(part_1(&ins), 1000000);
    }

    #[test]
    fn part_1_switch_on_all() {
        let ins = vec![Instruction::from("toggle 0,0 through 999,999")];
        assert_eq!(part_1(&ins), 1000000);
    }

    #[test]
    fn part_2_test_example_1() {
        let ins = vec![Instruction::from("turn on 0,0 through 0,0")];
        assert_eq!(part_2(&ins), 1);
    }

    #[test]
    fn part_2_test_example_2() {
        let ins = vec![Instruction::from("toggle 0,0 through 999,999")];
        assert_eq!(part_2(&ins), 2000000);
    }
}
