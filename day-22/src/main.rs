#![allow(dead_code)]
use regex::Regex;
use std::cmp;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::RangeInclusive;

fn main() {
    let input = load_input();
    println!("Solution for part 1: {}", solve(&input, true));
    println!("Solution for part 2: {}", solve(&input, false));
}

// Brute force for part 1
fn part_1(rules: &[Rule]) -> usize {
    let mut on_cubes: HashSet<(isize, isize, isize)> = HashSet::new();
    for rule in rules {
        if !rule.small {
            break;
        }
        for x in rule.cube.iter_x() {
            for y in rule.cube.iter_y() {
                for z in rule.cube.iter_z() {
                    if rule.cube.state {
                        on_cubes.insert((x, y, z));
                    } else {
                        on_cubes.remove(&(x, y, z));
                    }
                }
            }
        }
    }
    on_cubes.len()
}

fn solve(rules: &[Rule], small_only: bool) -> usize {
    let mut checked_cubes: Vec<Cube> = Vec::new();
    for rule in rules {
        if small_only && !rule.small {
            break;
        }
        let mut intersections: Vec<Cube> = Vec::new();
        for cube in &checked_cubes {
            if let Some(overlap) = rule.cube.intersects(cube) {
                intersections.push(overlap);
            }
        }
        checked_cubes.append(&mut intersections);

        if rule.cube.state {
            checked_cubes.push(rule.cube)
        }
    }
    let mut counter = 0;
    for cube in checked_cubes {
        match cube.state {
            true => counter += cube.get_volume(),
            false => counter -= cube.get_volume(),
        }
    }
    counter
}

fn load_input() -> Vec<Rule> {
    let file = File::open("input").expect("No input file found");
    let reader = BufReader::new(file);
    let reg = Regex::new(r"(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)..(-?\d+)")
        .unwrap();
    let mut rules: Vec<Rule> = Vec::new();
    for line in reader.lines() {
        let text = line.unwrap();
        let captures = reg.captures(&text).unwrap();
        let mut captures_iter = captures.iter();
        let state = match captures_iter.nth(1).unwrap().unwrap().as_str() {
            "on" => true,
            "off" => false,
            _ => unreachable!(),
        };
        let numbers: Vec<isize> = captures_iter
            .map(|x| x.unwrap().as_str().parse::<isize>().unwrap())
            .collect();
        rules.push(Rule::new(
            numbers[0], numbers[1], numbers[2], numbers[3], numbers[4], numbers[5], state,
        ));
    }
    rules
}

#[derive(Debug)]
struct Rule {
    small: bool,
    cube: Cube,
}

impl Rule {
    fn new(
        x_min: isize,
        x_max: isize,
        y_min: isize,
        y_max: isize,
        z_min: isize,
        z_max: isize,
        state: bool,
    ) -> Self {
        let small = x_min >= -50
            && x_max <= 50
            && y_min >= -50
            && y_max <= 50
            && z_min >= -50
            && z_max <= 50;
        Self {
            small,
            cube: Cube::new(x_min, x_max, y_min, y_max, z_min, z_max, state),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Cube {
    state: bool,
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
    z_min: isize,
    z_max: isize,
}

impl Cube {
    fn new(
        x_min: isize,
        x_max: isize,
        y_min: isize,
        y_max: isize,
        z_min: isize,
        z_max: isize,
        state: bool,
    ) -> Self {
        Self {
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
            state,
        }
    }
    fn iter_x(&self) -> RangeInclusive<isize> {
        self.x_min..=self.x_max
    }

    fn iter_y(&self) -> RangeInclusive<isize> {
        self.y_min..=self.y_max
    }

    fn iter_z(&self) -> RangeInclusive<isize> {
        self.z_min..=self.z_max
    }

    fn intersects(&self, other: &Cube) -> Option<Cube> {
        let x_overlap = self.x_min <= other.x_max && self.x_max >= other.x_min;
        let y_overlap = self.y_min <= other.y_max && self.y_max >= other.y_min;
        let z_overlap = self.z_min <= other.z_max && self.z_max >= other.z_min;
        if x_overlap && y_overlap && z_overlap {
            let x_min = cmp::max(self.x_min, other.x_min);
            let x_max = cmp::min(self.x_max, other.x_max);
            let y_min = cmp::max(self.y_min, other.y_min);
            let y_max = cmp::min(self.y_max, other.y_max);
            let z_min = cmp::max(self.z_min, other.z_min);
            let z_max = cmp::min(self.z_max, other.z_max);
            Some(Cube::new(
                x_min,
                x_max,
                y_min,
                y_max,
                z_min,
                z_max,
                !other.state,
            ))
        } else {
            None
        }
    }

    fn get_volume(&self) -> usize {
        ((self.x_max - self.x_min + 1)
            * (self.y_max - self.y_min + 1)
            * (self.z_max - self.z_min + 1))
            .try_into()
            .unwrap()
    }
}
