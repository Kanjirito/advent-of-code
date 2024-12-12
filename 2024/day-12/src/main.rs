use std::io::BufReader;
use std::{collections::HashSet, fs::File};

use utils::{BufReadExt, Grid, GridMaker, DIAGONAL};

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&input));
}

fn part_1(input: &Grid<char>) -> usize {
    let mut counter = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if visited.contains(&(x, y)) || input[y][x] == '×' {
                continue;
            } else {
                let before = visited.len();
                counter +=
                    find_perimeter(x, y, input, &mut visited).len() * (visited.len() - before);
            }
        }
    }

    counter
}

fn part_2(input: &Grid<char>) -> usize {
    let mut counter = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if visited.contains(&(x, y)) || input[y][x] == '×' {
                continue;
            } else {
                let before = visited.len();
                let perimeter = find_perimeter(x, y, input, &mut visited);
                counter += count_sides(perimeter) * (visited.len() - before);
            }
        }
    }

    counter
}

fn count_sides(mut perimeter: HashSet<(usize, usize, isize, isize)>) -> usize {
    let mut sides = 0;

    while let Some(&cur) = perimeter.iter().next() {
        perimeter.remove(&cur);
        sides += 1;

        let modif = if cur.3 == 0 {
            [(0, -1), (0, 1)]
        } else {
            [(-1, 0), (1, 0)]
        };

        for (mod_x, mod_y) in modif {
            let mut new_x = cur.0.saturating_add_signed(mod_x);
            let mut new_y = cur.1.saturating_add_signed(mod_y);
            while perimeter.remove(&(new_x, new_y, cur.2, cur.3)) {
                new_x = new_x.saturating_add_signed(mod_x);
                new_y = new_y.saturating_add_signed(mod_y);
            }
        }
    }
    sides
}

fn find_perimeter(
    x: usize,
    y: usize,
    grid: &Grid<char>,
    visited: &mut HashSet<(usize, usize)>,
) -> HashSet<(usize, usize, isize, isize)> {
    let mut perimenter = HashSet::new();

    let target = grid[y][x];
    let mut q = vec![(x, y)];

    while let Some((cur_x, cur_y)) = q.pop() {
        if visited.contains(&(cur_x, cur_y)) {
            continue;
        }
        visited.insert((cur_x, cur_y));

        for (m_x, m_y) in DIAGONAL {
            let new_x = cur_x.saturating_add_signed(m_x);
            let new_y = cur_y.saturating_add_signed(m_y);
            if grid[new_y][new_x] == target {
                q.push((new_x, new_y));
            } else {
                perimenter.insert((new_x, new_y, m_x, m_y));
            }
        }
    }

    perimenter
}

fn load_input(name: &str) -> Grid<char> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut grid = GridMaker::new('×');

    for line in reader.lines_unwrap() {
        grid.push(line.chars());
    }

    grid.finish()
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = load_input("example");
        assert_eq!(part_1(&input), 1930);
    }

    #[test]
    fn part_2_test() {
        let input = load_input("example");
        assert_eq!(part_2(&input), 1206);
    }
}
