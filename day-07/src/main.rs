use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Clone)]
enum Connection {
    And(Target, Target),
    Or(Target, Target),
    Not(Target),
    RShift(Target, u16),
    LShift(Target, u16),
    Single(Target),
}

impl From<&str> for Connection {
    fn from(value: &str) -> Connection {
        let split: Vec<&str> = value.split(' ').collect();
        if split.len() == 1 {
            Self::Single(Target::from(split[0]))
        } else if split.len() == 2 {
            Self::Not(Target::from(split[1]))
        } else {
            match split[1] {
                "AND" => Self::And(Target::from(split[0]), Target::from(split[2])),
                "OR" => Self::Or(Target::from(split[0]), Target::from(split[2])),
                "RSHIFT" => Self::RShift(Target::from(split[0]), split[2].parse().unwrap()),
                "LSHIFT" => Self::LShift(Target::from(split[0]), split[2].parse().unwrap()),
                _ => unreachable!(),
            }
        }
    }
}

impl From<u16> for Connection {
    fn from(value: u16) -> Connection {
        Self::Single(Target::Num(value))
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
enum Target {
    Name(String),
    Num(u16),
}

impl From<&str> for Target {
    fn from(value: &str) -> Self {
        if let Ok(num) = value.parse() {
            Self::Num(num)
        } else {
            Self::Name(value.to_owned())
        }
    }
}

fn main() {
    let input = load_input("input");
    let mut copy = input.clone();
    let new_value = part_1(&mut copy, Target::from("a"));
    println!("Solution for part 1: {}", new_value);
    println!("Solution for part 2: {}", part_2(input, new_value));
}

fn part_1(ins: &mut HashMap<Target, Connection>, target: Target) -> u16 {
    // Numbers don't link to anything, just return the value
    if let Target::Num(n) = target {
        return n;
    }

    let value = match ins
        .get(&target)
        .expect("All wires are expected to be connected to something")
        .clone()
    {
        Connection::And(one, two) => part_1(ins, one) & part_1(ins, two),
        Connection::Or(one, two) => part_1(ins, one) | part_1(ins, two),
        Connection::Not(v) => !part_1(ins, v),
        Connection::RShift(v, c) => part_1(ins, v) >> c,
        Connection::LShift(v, c) => part_1(ins, v) << c,
        Connection::Single(new) => part_1(ins, new),
    };

    // We now know what the value for the current target is so just set it to the number for future
    // searches
    ins.insert(target, value.into());
    value
}

fn part_2(mut ins: HashMap<Target, Connection>, new_value: u16) -> u16 {
    ins.insert(Target::Name("b".to_owned()), new_value.into());
    part_1(&mut ins, Target::Name("a".to_owned()))
}

fn load_input(name: &str) -> HashMap<Target, Connection> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut instructions = HashMap::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        let mut split = line.split(" -> ");
        let bit = Connection::from(split.next().unwrap());
        let target = split.next().unwrap().into();
        instructions.insert(target, bit);
    }
    instructions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let mut input = load_input("example");
        eprintln!("d");
        assert_eq!(part_1(&mut input, Target::from("d")), 72);
        eprintln!("e");
        assert_eq!(part_1(&mut input, Target::from("e")), 507);
        eprintln!("f");
        assert_eq!(part_1(&mut input, Target::from("f")), 492);
        eprintln!("g");
        assert_eq!(part_1(&mut input, Target::from("g")), 114);
        eprintln!("h");
        assert_eq!(part_1(&mut input, Target::from("h")), 65412);
        eprintln!("i");
        assert_eq!(part_1(&mut input, Target::from("i")), 65079);
        eprintln!("j");
        assert_eq!(part_1(&mut input, Target::from("x")), 123);
        eprintln!("k");
        assert_eq!(part_1(&mut input, Target::from("y")), 456);
    }
}
