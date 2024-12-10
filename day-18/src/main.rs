use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let (cubes, min_x, max_x, min_y, max_y, min_z, max_z) = load_input("input");
    println!("Part 1: {}", part_1(&cubes));
    println!(
        "Part 2: {}",
        part_2(min_x, max_x, min_y, max_y, min_z, max_z, &cubes)
    )
}

/// Solution for part 1
///
/// Just iterates over each cube, counts how many sides each cube has exposed and adds them all together.
fn part_1(cubes: &HashSet<Cube>) -> i64 {
    let mut counter = 0;
    for cube in cubes {
        counter += cube.count_sides(cubes)
    }
    counter
}

/// Solution for part 2
///
/// Does a DFS from outside of the droplet. If it gets to a droplet it means one side of a cube is exposed.
fn part_2(
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    min_z: i64,
    max_z: i64,
    cubes: &HashSet<Cube>,
) -> i64 {
    let mut counter = 0;
    // Keeps track of what was already checked
    let mut visited: HashSet<(i64, i64, i64)> = HashSet::new();
    // Add the starting point
    visited.insert((max_x, max_y, max_z));
    let mut q: Vec<(i64, i64, i64)> = vec![(max_x, max_y, max_z)];
    while let Some(cur) = q.pop() {
        // Iterates over every neighbour of the current position
        for (x_off, y_off, z_off) in [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ] {
            let x = cur.0 + x_off;
            let y = cur.1 + y_off;
            let z = cur.2 + z_off;
            // If the neighbour has not yet been checked and isn't past the "border"...
            if !visited.contains(&(x, y, z))
                && x >= min_x
                && x <= max_x
                && y >= min_y
                && y <= max_y
                && z >= min_z
                && z <= max_z
            {   
                // ...make a temporary cube and check if it's part of the droplet
                let cube = Cube { x, y, z };
                if cubes.contains(&cube) {
                    // If yes, that means a side is exposed so increase the counter
                    counter += 1;
                    // Continue is needed because you don't want to actually move into the droplet and the same droplet
                    // cube might be exposed from multiple sides
                    continue;
                }
                // Mark the neighbour as visited and add it to the queue
                visited.insert((x, y, z));
                q.push((x, y, z));
            }
        }
    }
    counter
}

fn load_input(name: &str) -> (HashSet<Cube>, i64, i64, i64, i64, i64, i64) {
    let file = File::open(name).expect("No input file found");
    let reader = BufReader::new(file);
    let mut cubes: HashSet<Cube> = HashSet::new();
    // Keep track of the highest and lowest value in each direction
    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut min_y = i64::MAX;
    let mut max_y = i64::MIN;
    let mut min_z = i64::MAX;
    let mut max_z = i64::MIN;
    for line in reader.lines().map(|l| l.unwrap()) {
        let cords: Vec<i64> = line
            .split(',')
            .into_iter()
            .map(|x| x.parse().unwrap())
            .collect();
        min_x = min_x.min(cords[0]);
        max_x = max_x.max(cords[0]);
        min_y = min_y.min(cords[1]);
        max_y = max_y.max(cords[1]);
        min_z = min_z.min(cords[2]);
        max_z = max_z.max(cords[2]);
        let cube = Cube {
            x: cords[0],
            y: cords[1],
            z: cords[2],
        };
        cubes.insert(cube);
    }
    // Increase the min and max values so that they are past the droplet
    (
        cubes,
        min_x - 1,
        max_x + 1,
        min_y - 1,
        max_y + 1,
        min_z - 1,
        max_z + 1,
    )
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Cube {
    x: i64,
    y: i64,
    z: i64,
}

impl Cube {
    /// Counts how many of it's sides are exposed
    fn count_sides(&self, cubes: &HashSet<Cube>) -> i64 {
        // Start with all sides exposed
        let mut sides = 6;
        // Iterate over every possible neighbour
        for (x_off, y_off, z_off) in [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ] {
            // Make a temporary cube
            let other_cube = Cube {
                x: self.x + x_off,
                y: self.y + y_off,
                z: self.z + z_off,
            };
            // If it's an actual cube it means that 1 side of the cube is covered
            if cubes.contains(&other_cube) {
                sides -= 1;
            }
        }
        sides
    }
}

#[test]
fn example() {
    let (cubes, min_x, max_x, min_y, max_y, min_z, max_z) = load_input("example");
    assert_eq!(part_1(&cubes), 64);
    assert_eq!(part_2(min_x, max_x, min_y, max_y, min_z, max_z, &cubes), 58)
}
