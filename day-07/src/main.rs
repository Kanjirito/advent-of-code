use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Input = Vec<(usize, Vec<usize>)>;

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&input));
}

fn part_1(input: &Input) -> usize {
    let mut counter = 0;

    for (target, values) in input {
        if check_line(*target, values, &[Operation::Add, Operation::Mult]) {
            counter += target;
        }
    }

    counter
}

fn part_2(input: &Input) -> usize {
    let mut counter = 0;

    for (target, values) in input {
        if check_line(
            *target,
            values,
            &[Operation::Add, Operation::Mult, Operation::Combine],
        ) {
            counter += target;
        }
    }

    counter
}

fn check_line(target: usize, values: &[usize], ops: &[Operation]) -> bool {
    let mut q: Vec<(usize, &[usize])> = vec![(0, values)];

    while let Some((cur, left_values)) = q.pop() {
        if left_values.is_empty() {
            if cur == target {
                return true;
            } else {
                continue;
            }
        }
        for op in ops {
            let new_value = op.do_math(cur, left_values[0]);
            if new_value > target {
                continue;
            }
            q.push((new_value, &left_values[1..]));
        }
    }
    false
}

fn load_input(name: &str) -> Input {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut lines = vec![];

    for line in reader.lines().map(|l| l.unwrap()) {
        let mut split = line.split(": ");
        let target = split.next().unwrap().parse().unwrap();
        let values = split
            .next()
            .unwrap()
            .split(' ')
            .map(|n| n.parse().unwrap())
            .collect();
        lines.push((target, values));
    }
    lines
}

#[derive(Debug)]
enum Operation {
    Add,
    Mult,
    Combine,
}

impl Operation {
    fn do_math(&self, first: usize, second: usize) -> usize {
        match self {
            Operation::Add => first + second,
            Operation::Mult => first * second,
            Operation::Combine => format!("{first}{second}").parse().unwrap(),
        }
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = load_input("example");
        assert_eq!(part_1(&input), 3749);
    }

    #[test]
    fn part_2_test() {
        let input = load_input("example");
        assert_eq!(part_2(&input), 11387);
    }
}
