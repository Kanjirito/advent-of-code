use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", solve(&input, 100, false));
    println!("Solution for part 2: {}", solve(&input, 100, true));
}

fn solve(grid: &[Vec<bool>], steps: usize, part_2: bool) -> usize {
    let rows = grid.len();
    let columns = grid[0].len();
    let corners = [
        (1, 1),
        (1, columns - 2),
        (rows - 2, 1),
        (rows - 2, columns - 2),
    ];
    let mut cur = grid.to_vec();

    if part_2 {
        for (y, x) in corners {
            cur[y][x] = true;
        }
    }

    for _ in 0..steps {
        let mut tmp = cur.clone();
        for y in 1..(rows - 1) {
            for x in 1..(columns - 1) {
                if part_2 && corners.contains(&(y, x)) {
                    continue;
                }
                let neighbours = get_alive_neighbours(&cur, x, y);
                if cur[y][x] {
                    if !(neighbours == 2 || neighbours == 3) {
                        tmp[y][x] = false;
                    }
                } else {
                    tmp[y][x] = neighbours == 3;
                }
            }
        }
        cur = tmp;
    }

    cur.into_iter()
        .take(rows - 1)
        .skip(1)
        .flat_map(|r| r.into_iter().take(columns - 1).skip(1))
        .filter(|l| *l)
        .count()
}

fn get_alive_neighbours(grid: &[Vec<bool>], x: usize, y: usize) -> usize {
    let mut counter = 0;

    #[allow(clippy::needless_range_loop)]
    for new_y in (y - 1)..=(y + 1) {
        for new_x in (x - 1)..=(x + 1) {
            if new_x == x && new_y == y {
                continue;
            }
            if grid[new_y][new_x] {
                counter += 1;
            }
        }
    }

    counter
}

#[allow(dead_code)]
fn print_grid(grid: &[Vec<bool>]) {
    for row in grid.iter().take(grid.len() - 1).skip(1) {
        for ele in row.iter().take(grid.len() - 1).skip(1) {
            if *ele {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!();
    }
}

fn load_input(name: &str) -> Vec<Vec<bool>> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut grid = vec![vec![]];
    for line in reader.lines().map(|l| l.unwrap()) {
        let mut row = vec![false];
        for c in line.chars() {
            match c {
                '#' => row.push(true),
                '.' => row.push(false),
                _ => unreachable!(),
            }
        }
        row.push(false);
        grid.push(row);
    }
    grid.push(vec![false; grid[1].len()]);
    grid[0] = vec![false; grid[1].len()];
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = load_input("example");
        assert_eq!(solve(&input, 4, false), 4)
    }

    #[test]
    fn part_2_test() {
        let input = load_input("example");
        assert_eq!(solve(&input, 5, true), 17)
    }
}
