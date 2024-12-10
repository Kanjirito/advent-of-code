use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Grid = Vec<Vec<char>>;

const DIRES: [(isize, isize); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, -1),
    (-1, 1),
];

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&input));
}

fn part_1(grid: &Grid) -> u64 {
    let mut counter = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'X' {
                for modif in DIRES {
                    if check_directions(grid, (y, x), modif) {
                        counter += 1;
                    }
                }
            }
        }
    }
    counter
}

fn part_2(grid: &Grid) -> u64 {
    let mut counter = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'A' && check_x(grid, (y, x)) {
                counter += 1;
            }
        }
    }
    counter
}

fn check_x(grid: &Grid, start: (usize, usize)) -> bool {
    let y = start.0;
    let x = start.1;
    ((grid[y - 1][x - 1] == 'M' && grid[y + 1][x + 1] == 'S')
        || (grid[y - 1][x - 1] == 'S' && grid[y + 1][x + 1] == 'M'))
        && ((grid[y - 1][x + 1] == 'M' && grid[y + 1][x - 1] == 'S')
            || (grid[y - 1][x + 1] == 'S' && grid[y + 1][x - 1] == 'M'))
}

fn check_directions(grid: &Grid, start: (usize, usize), modif: (isize, isize)) -> bool {
    let mut y = start.0 as isize;
    let mut x = start.1 as isize;

    for letter in ['M', 'A', 'S'] {
        y += modif.0;
        x += modif.1;

        if grid[y as usize][x as usize] != letter {
            return false;
        }
    }
    true
}

#[allow(dead_code)]
fn print_grid(grid: &Grid) {
    for line in grid {
        for c in line {
            print!("{c}");
        }
        println!()
    }
}

fn load_input(name: &str) -> Grid {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut grid = vec![vec![]];

    for line in reader.lines().map(|l| l.unwrap()) {
        let mut cur = vec!['.'];
        cur.extend(line.chars());
        cur.push('.');
        grid.push(cur);
    }
    grid[0] = vec!['.'; grid[1].len()];
    grid.push(vec!['.'; grid[1].len()]);
    grid
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = load_input("example");
        assert_eq!(part_1(&input), 18);
    }

    #[test]
    fn part_2_test() {
        let input = load_input("example");
        assert_eq!(part_2(&input), 9);
    }
}
