use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Grid = Vec<Vec<usize>>;

const MOVES: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn main() {
    let (starts, grid) = load_input("input");
    println!("Solution for part 1: {}", part_1(&starts, &grid));
    println!("Solution for part 2: {}", part_2(&starts, &grid));
}

fn part_1(starts: &[(usize, usize)], grid: &Grid) -> usize {
    let mut counter = 0;

    for start in starts {
        counter += find_ends(*start, grid);
    }

    counter
}

fn part_2(starts: &[(usize, usize)], grid: &Grid) -> usize {
    let mut counter = 0;

    let mut q = starts.to_vec();

    while let Some((cur_x, cur_y)) = q.pop() {
        let cur_value = grid[cur_y][cur_x];

        for (mod_x, mod_y) in MOVES {
            let new_x = cur_x.saturating_add_signed(mod_x);
            let new_y = cur_y.saturating_add_signed(mod_y);
            let new_value = grid[new_y][new_x];

            if new_value == 10 {
                continue;
            } else if new_value == cur_value + 1 {
                if new_value == 9 {
                    counter += 1;
                    continue;
                }
                q.push((new_x, new_y));
            }
        }
    }
    counter
}

fn find_ends(start: (usize, usize), grid: &Grid) -> usize {
    let mut visited = HashSet::new();

    let mut q = vec![start];

    while let Some((cur_x, cur_y)) = q.pop() {
        let cur_value = grid[cur_y][cur_x];

        for (mod_x, mod_y) in MOVES {
            let new_x = cur_x.saturating_add_signed(mod_x);
            let new_y = cur_y.saturating_add_signed(mod_y);
            let new_value = grid[new_y][new_x];

            if new_value == 10 {
                continue;
            } else if new_value == cur_value + 1 {
                if new_value == 9 {
                    visited.insert((new_x, new_y));
                    continue;
                }
                q.push((new_x, new_y));
            }
        }
    }

    visited.len()
}

fn load_input(name: &str) -> (Vec<(usize, usize)>, Grid) {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut grid = vec![vec![]];
    let mut starts = vec![];

    for (y, line) in reader.lines().map(|l| l.unwrap()).enumerate() {
        let mut cur = vec![10];
        for (x, n) in line
            .chars()
            .map(|x| x.to_digit(10).unwrap() as usize)
            .enumerate()
        {
            cur.push(n);
            if n == 0 {
                starts.push((x + 1, y + 1));
            }
        }
        cur.push(10);
        grid.push(cur);
    }

    grid[0] = vec![10; grid[1].len()];
    grid.push(vec![10; grid[1].len()]);
    (starts, grid)
}

#[allow(dead_code)]
fn print_grid(grid: &Grid) {
    for line in grid {
        for t in line {
            if *t == 10 {
                print!("X");
            } else {
                print!("{}", t);
            }
        }
        println!();
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let (starts, grid) = load_input("example");
        assert_eq!(part_1(&starts, &grid), 36);
    }

    #[test]
    fn part_2_test() {
        let (starts, grid) = load_input("example");
        assert_eq!(part_2(&starts, &grid), 81);
    }
}
