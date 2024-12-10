use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::Neg;

fn main() {
    let mut input = load_input();
    let mut solved: Vec<Scanner> = Vec::new();

    // Setup the first scanner as the base
    let mut default_rules = HashMap::new();
    default_rules.insert('x', (Sign::Same, 'x'));
    default_rules.insert('y', (Sign::Same, 'y'));
    default_rules.insert('z', (Sign::Same, 'z'));
    input[0].change_cords(0, 0, 0, &default_rules);
    solved.push(input.pop_front().unwrap());

    'main: while let Some(mut scanner) = input.pop_front() {
        for solved_scanner in solved.iter() {
            if solved_scanner.matches(&scanner) {
                scanner.match_rotation(solved_scanner);
                solved.push(scanner);
                continue 'main;
            }
        }
        input.push_back(scanner);
    }
    let mut beacons: HashSet<(isize, isize, isize)> = HashSet::new();
    for scanner in &solved {
        for beacon in &scanner.beacons {
            beacons.insert((beacon.x.unwrap(), beacon.y.unwrap(), beacon.z.unwrap()));
        }
    }
    println!("Solution for part 1: {}", beacons.len());

    let mut biggest_manhattan: usize = 0;
    for x in 0..solved.len() {
        for y in 0..solved.len() {
            if x == y {
                continue;
            }
            let first = &solved[x];
            let second = &solved[y];
            let man = get_absolute_diff(first.x.unwrap(), second.x.unwrap())
                + get_absolute_diff(first.y.unwrap(), second.y.unwrap())
                + get_absolute_diff(first.z.unwrap(), second.z.unwrap());
            if man > biggest_manhattan {
                biggest_manhattan = man;
            }
        }
    }
    println!("Solution for part 2: {}", biggest_manhattan);
}

fn get_absolute_diff(first: isize, second: isize) -> usize {
    (first - second).abs().try_into().unwrap()
}

fn load_input() -> VecDeque<Scanner> {
    let file = File::open("input").expect("No input file found");
    let reader = BufReader::new(file);
    let mut scanners: VecDeque<Scanner> = VecDeque::new();
    let mut current_scanner = Scanner::new();
    for line in reader.lines() {
        let text = line.unwrap();
        if text.is_empty() {
            scanners.push_back(current_scanner);
            current_scanner = Scanner::new();
            continue;
        } else if text.starts_with("---") {
            continue;
        }
        let nums: Vec<isize> = text.split(',').map(|x| x.parse().unwrap()).collect();
        current_scanner.add_beacon(nums[0], nums[1], nums[2]);
    }
    scanners.push_back(current_scanner);
    scanners
}

#[derive(Debug)]
struct Scanner {
    x: Option<isize>,
    y: Option<isize>,
    z: Option<isize>,
    beacons: Vec<Beacon>,
}

impl Scanner {
    fn new() -> Self {
        Self {
            x: None,
            y: None,
            z: None,
            beacons: Vec::new(),
        }
    }

    /// Adds a new beacon to the sensor
    ///
    /// Automatically adds the new distance to all previous beacons.
    fn add_beacon(&mut self, relative_x: isize, relative_y: isize, relative_z: isize) {
        let mut new_beacon = Beacon::new(relative_x, relative_y, relative_z);
        for beacon in self.beacons.iter_mut() {
            let mut distance = vec![
                get_absolute_diff(beacon.relative_x, new_beacon.relative_x),
                get_absolute_diff(beacon.relative_y, new_beacon.relative_y),
                get_absolute_diff(beacon.relative_z, new_beacon.relative_z),
            ];
            distance.sort_unstable();
            beacon.add_distance(&distance);
            new_beacon.add_distance(&distance);
        }
        self.beacons.push(new_beacon);
    }

    /// Changes the actual cords to x, y, z and updates it's beacons
    fn change_cords(&mut self, x: isize, y: isize, z: isize, rules: &HashMap<char, (Sign, char)>) {
        self.x = Some(x);
        self.y = Some(y);
        self.z = Some(z);
        for beacon in self.beacons.iter_mut() {
            let mut correct_cords: Vec<isize> = Vec::with_capacity(3);
            for cord in ['x', 'y', 'z'].iter() {
                let mut num = match cord {
                    'x' => x,
                    'y' => y,
                    'z' => z,
                    _ => unreachable!(),
                };
                num += match rules[cord] {
                    (Sign::Same, c) => match c {
                        'x' => beacon.relative_x,
                        'y' => beacon.relative_y,
                        'z' => beacon.relative_z,
                        _ => unreachable!(),
                    },
                    (Sign::Opposite, c) => match c {
                        'x' => beacon.relative_x.neg(),
                        'y' => beacon.relative_y.neg(),
                        'z' => beacon.relative_z.neg(),
                        _ => unreachable!(),
                    },
                };
                correct_cords.push(num);
            }
            beacon.change_cords(correct_cords[0], correct_cords[1], correct_cords[2]);
        }
    }

    /// Checks if given Scanners are overlapping
    ///
    /// (12 beacons * 11 matching distances) / 2 to remove duplicates = 66 needed matching distances for 12 beacons to be common.
    fn matches(&self, other: &Self) -> bool {
        let mut beacon_match_counter = 0;
        for other_beacon in &other.beacons {
            for beacon in &self.beacons {
                for distance in &beacon.distances {
                    if other_beacon.distances.contains(distance) {
                        beacon_match_counter += 1;
                    }
                    if beacon_match_counter >= 66 {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Matches the rotation of self to other Scanner and calculates it's coordinates
    fn match_rotation(&mut self, other: &Self) {
        // Find 2 shared beacons
        let mut valid_pairs: Vec<(&Beacon, &Beacon)> = Vec::with_capacity(2);
        'main: for beacon in &self.beacons {
            for other_beacon in &other.beacons {
                if beacon.get_matches(other_beacon) >= 11 {
                    valid_pairs.push((other_beacon, beacon));
                    if valid_pairs.len() == 2 {
                        break 'main;
                    }
                }
            }
        }

        // Calculate the differences between 2 beacons of the same scanner for both scanners
        let first_offset = vec![
            valid_pairs[0].0.x.unwrap() - valid_pairs[1].0.x.unwrap(),
            valid_pairs[0].0.y.unwrap() - valid_pairs[1].0.y.unwrap(),
            valid_pairs[0].0.z.unwrap() - valid_pairs[1].0.z.unwrap(),
        ];
        let second_offset = vec![
            valid_pairs[0].1.relative_x - valid_pairs[1].1.relative_x,
            valid_pairs[0].1.relative_y - valid_pairs[1].1.relative_y,
            valid_pairs[0].1.relative_z - valid_pairs[1].1.relative_z,
        ];

        // Create the "rules" for how the coordinates map relative to the other Beacon.
        // Since both offset use the same Beacons the numbers will be the same but they might have
        // switched place and changed to negative.
        let mut rules: HashMap<char, (Sign, char)> = HashMap::with_capacity(3);
        for (i, c) in ['x', 'y', 'z'].iter().enumerate() {
            if first_offset[i].abs() == second_offset[0].abs() {
                rules.insert(
                    *c,
                    (Sign::from_numbers(&first_offset[i], &second_offset[0]), 'x'),
                );
            } else if first_offset[i].abs() == second_offset[1].abs() {
                rules.insert(
                    *c,
                    (Sign::from_numbers(&first_offset[i], &second_offset[1]), 'y'),
                );
            } else if first_offset[i].abs() == second_offset[2].abs() {
                rules.insert(
                    *c,
                    (Sign::from_numbers(&first_offset[i], &second_offset[2]), 'z'),
                );
            };
        }

        // Use the rules to the scanner's position
        let mut correct_cords: Vec<isize> = Vec::with_capacity(3);
        for cord in ['x', 'y', 'z'].iter() {
            let mut num = match cord {
                'x' => valid_pairs[0].0.x.unwrap(),
                'y' => valid_pairs[0].0.y.unwrap(),
                'z' => valid_pairs[0].0.z.unwrap(),
                _ => unreachable!(),
            };
            num -= match rules[cord] {
                (Sign::Same, c) => match c {
                    'x' => valid_pairs[0].1.relative_x,
                    'y' => valid_pairs[0].1.relative_y,
                    'z' => valid_pairs[0].1.relative_z,
                    _ => unreachable!(),
                },
                (Sign::Opposite, c) => match c {
                    'x' => valid_pairs[0].1.relative_x.neg(),
                    'y' => valid_pairs[0].1.relative_y.neg(),
                    'z' => valid_pairs[0].1.relative_z.neg(),
                    _ => unreachable!(),
                },
            };
            correct_cords.push(num);
        }

        self.change_cords(correct_cords[0], correct_cords[1], correct_cords[2], &rules);
    }
}

#[derive(Debug)]
enum Sign {
    Same,
    Opposite,
}

impl Sign {
    fn from_numbers(first: &isize, second: &isize) -> Self {
        if first.signum() != second.signum() {
            Self::Opposite
        } else {
            Self::Same
        }
    }
}

#[derive(Debug)]
struct Beacon {
    x: Option<isize>,
    y: Option<isize>,
    z: Option<isize>,
    relative_x: isize,
    relative_y: isize,
    relative_z: isize,
    /// Distances to other Beacons in current scanner
    distances: HashSet<(usize, usize, usize)>,
}

impl Beacon {
    fn new(relative_x: isize, relative_y: isize, relative_z: isize) -> Self {
        Self {
            x: None,
            y: None,
            z: None,
            relative_x,
            relative_y,
            relative_z,
            distances: HashSet::new(),
        }
    }

    fn add_distance(&mut self, distance: &[usize]) {
        self.distances
            .insert((distance[0], distance[1], distance[2]));
    }

    fn change_cords(&mut self, x: isize, y: isize, z: isize) {
        self.x = Some(x);
        self.y = Some(y);
        self.z = Some(z);
    }

    fn get_matches(&self, other: &Self) -> usize {
        self.distances
            .iter()
            .filter(|x| other.distances.contains(x))
            .count()
    }
}

#[test]
fn test_absolutes() {
    assert_eq!(get_absolute_diff(10, 5), 5);
    assert_eq!(get_absolute_diff(-5, -10), 5);
    assert_eq!(get_absolute_diff(10, -5), 15);
    assert_eq!(get_absolute_diff(0, 10), 10);
    assert_eq!(get_absolute_diff(10, 0), 10);
    assert_eq!(get_absolute_diff(5, 5), 0);
}
