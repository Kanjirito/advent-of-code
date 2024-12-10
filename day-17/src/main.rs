use regex::Match;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Neg;

fn main() {
    let target = load_input();
    let min_x_velocity = find_lowest_x(target.x_1);
    // It can't go so fast that it overshoots the target in the first jump
    let max_x_velocity = target.x_2;
    // It comes down as fast as it comes up so when shooting with y_velocity=10 once it comes back to y=0
    // it will have the same y_velocity as if shooting with y_velocity=-10
    // And y_veloctiy can't be high enough to overshoot the target in 1 step
    let max_y_velocity = target.y_2.abs();
    let results = simulate_shots(&target, min_x_velocity, max_x_velocity, max_y_velocity);
    println!("Solution for part 1: {}", results.0);
    println!("Solution for part 2: {}", results.1);
}

fn find_lowest_x(target: isize) -> isize {
    // x velocity range is a triangular number so we just find the first that reaches our target
    // everything smaller than that just won't reach
    let mut counter = 0;
    for n in 1.. {
        counter += n;
        if counter >= target {
            return n;
        }
    }
    unreachable!()
}

#[allow(clippy::comparison_chain)]
fn simulate_shots(
    target: &Target,
    min_x_velocity: isize,
    max_x_velocity: isize,
    max_y_velocity: isize,
) -> (isize, usize) {
    let mut valid_velocities: HashMap<(isize, isize), isize> = HashMap::new();
    for x in min_x_velocity..=max_x_velocity {
        for y in max_y_velocity.neg()..=max_y_velocity {
            let mut current_x = 0;
            let mut current_y = 0;

            let mut current_x_vel = x;
            let mut current_y_vel = y;

            let mut current_highest_y = 0;
            loop {
                current_x += current_x_vel;
                current_y += current_y_vel;

                // Keep track of the highest it's been
                if current_y > current_highest_y {
                    current_highest_y = current_y;
                }

                // Since it's a trench the target will always be after the highest point it's safe to do this
                if target.contains(current_x, current_y) {
                    valid_velocities.insert((x, y), current_highest_y);
                    break;
                }

                // It's past the target
                if current_y < target.y_2 || current_x > target.x_2 {
                    break;
                }

                // Change the velocity
                if current_x_vel > 0 {
                    current_x_vel -= 1;
                } else if current_x_vel < 0 {
                    current_x_vel += 1;
                }
                current_y_vel -= 1;
            }
        }
    }
    (
        *valid_velocities.values().max().unwrap(),
        valid_velocities.len(),
    )
}

fn load_input() -> Target {
    let mut file = File::open("input").expect("No input file found");
    let mut line = String::new();
    file.read_to_string(&mut line).unwrap();
    let re = Regex::new(r"x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)").unwrap();
    let re_match = re.captures(&line).unwrap();
    Target {
        x_1: handle_match(re_match.get(1)),
        x_2: handle_match(re_match.get(2)),
        y_1: handle_match(re_match.get(4)),
        y_2: handle_match(re_match.get(3)),
    }
}

fn handle_match(m: Option<Match>) -> isize {
    m.unwrap().as_str().parse().unwrap()
}

#[derive(Debug)]
struct Target {
    x_1: isize,
    x_2: isize,
    y_1: isize,
    y_2: isize,
}

#[allow(dead_code)]
impl Target {
    fn contains(&self, x: isize, y: isize) -> bool {
        self.contains_x(x) && self.contains_y(y)
    }

    fn contains_y(&self, y: isize) -> bool {
        self.y_1 >= y && y >= self.y_2
    }

    fn contains_x(&self, x: isize) -> bool {
        self.x_1 <= x && x <= self.x_2
    }
}
