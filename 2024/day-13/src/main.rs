use std::fs::File;
use std::io::BufReader;

use utils::{divmod, lcm, BufReadExt};

/// ((AX, AY), (BX, BY), (PX, PY))
type Machine = ((usize, usize), (usize, usize), (usize, usize));

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&input));
}

fn part_1(input: &[Machine]) -> usize {
    let mut counter = 0;
    for machine in input {
        if let Some((a, b)) = solve_math(*machine) {
            counter += (a * 3) + b;
        }
    }

    counter
}

fn part_2(input: &[Machine]) -> usize {
    let mut counter = 0;
    for machine in input {
        let new_target_x = 10000000000000 + machine.2 .0;
        let new_target_y = 10000000000000 + machine.2 .1;
        let new_machine = (
            (machine.0 .0, machine.0 .1),
            (machine.1 .0, machine.1 .1),
            (new_target_x, new_target_y),
        );
        let r = solve_math(new_machine);
        if let Some((a, b)) = r {
            counter += (a * 3) + b;
        }
    }

    counter
}

/// Finds the solution by solving the equation system.
///
/// ```text
/// (A * AX) + (B * BX) = PRIZE_X
/// (A * AY) + (B * BY) = PRIZE_Y
/// ```
/// Where:
/// - A: the times the A button is pressed
/// - B: the times the B button is pressed
/// - AX: how far the A button moves horizontally
/// - BX: how far the B button moves horizontally
/// - AY: how far the A button moves vertically
/// - BY: how far the B button moves vertically
/// - PRIZE_X/PRIZE_Y: the location of the prize
fn solve_math(machine: Machine) -> Option<(usize, usize)> {
    let ((a_x, a_y), (b_x, b_y), (target_x, target_y)) = machine;

    let a_lcm = lcm(a_x, a_y);

    // Removes A from both of the equations
    let eq_1 = ((a_lcm / a_x) * b_x, (a_lcm / a_x) * target_x);
    let eq_2 = ((a_lcm / a_y) * b_y, (a_lcm / a_y) * target_y);

    // Subtracts the equations so you end up with
    // B * BX = XXXX
    // Where XXXX is some regular number
    let cur = (eq_1.0.abs_diff(eq_2.0), eq_1.1.abs_diff(eq_2.1));

    // Divides both sides by BX to get B = XXXX
    let (b_value, rem) = divmod(cur.1, cur.0);

    // If it doesn't divide cleanly then it's not a whole number so we can't get there
    if rem != 0 {
        return None;
    };

    // Replace B with the actual value in one of the equations and solve it to get A
    let cur = (a_x, target_x - (b_value * b_x));

    let (a_value, rem) = divmod(cur.1, cur.0);

    if rem != 0 {
        return None;
    };
    Some((a_value, b_value))
}

fn load_input(name: &str) -> Vec<Machine> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut result = vec![];
    let mut lines = reader.lines_unwrap();

    while let Some(a_line) = lines.next() {
        let mut split_a = a_line.strip_prefix("Button A: X+").unwrap().split(", Y+");

        let b_line = lines.next().unwrap();
        let mut split_b = b_line.strip_prefix("Button B: X+").unwrap().split(", Y+");

        let prize_line = lines.next().unwrap();
        let mut prize_split = prize_line.strip_prefix("Prize: X=").unwrap().split(", Y=");

        result.push((
            (
                split_a.next().unwrap().parse::<usize>().unwrap(),
                split_a.next().unwrap().parse::<usize>().unwrap(),
            ),
            (
                split_b.next().unwrap().parse::<usize>().unwrap(),
                split_b.next().unwrap().parse::<usize>().unwrap(),
            ),
            (
                prize_split.next().unwrap().parse::<usize>().unwrap(),
                prize_split.next().unwrap().parse::<usize>().unwrap(),
            ),
        ));
        lines.next();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = load_input("example");
        assert_eq!(part_1(&input), 480);
    }

    #[test]
    fn part_2_test() {
        let input = load_input("example");
        assert_eq!(part_2(&input), 875318608908);
    }
}
