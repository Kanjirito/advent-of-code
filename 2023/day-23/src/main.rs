use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
}

fn part_1(map: &[Vec<char>]) -> usize {
    find_longest_path(map, (1, 1), (map[0].len() - 2, map.len() - 1), false)
}

fn part_2(map: &[Vec<char>]) -> usize {
    find_longest_path(map, (1, 1), (map[0].len() - 2, map.len() - 1), true)
}

fn find_longest_path(
    map: &[Vec<char>],
    start: (usize, usize),
    end: (usize, usize),
    no_slopes: bool,
) -> usize {
    let mut cur_lonest = 0;
    let mut q = vec![Ele::new(start.0, start.1)];

    while let Some(cur) = q.pop() {
        if cur.x == end.0 && cur.y == end.1 {
            cur_lonest = cur_lonest.max(cur.cur_dist);
            continue;
        }
        let mut cur_char = map[cur.y][cur.x];
        if no_slopes {
            cur_char = '.';
        }

        let mut neighbors: Vec<(usize, usize)> = Vec::new();
        match cur_char {
            '^' => {
                neighbors.push((cur.x, cur.y - 1));
            }
            '>' => {
                neighbors.push((cur.x + 1, cur.y));
            }
            'v' => {
                neighbors.push((cur.x, cur.y + 1));
            }
            '<' => {
                neighbors.push((cur.x - 1, cur.y));
            }
            '.' => {
                for (new_x, new_y) in [
                    (cur.x + 1, cur.y),
                    (cur.x - 1, cur.y),
                    (cur.x, cur.y + 1),
                    (cur.x, cur.y - 1),
                ] {
                    neighbors.push((new_x, new_y));
                }
            }
            _ => unreachable!(),
        }

        for (new_x, new_y) in neighbors {
            if !cur.visited.contains(&(new_x, new_y)) && map[new_y][new_x] != '#' {
                q.push(Ele::new_from(&cur, new_x, new_y));
            }
        }
    }
    cur_lonest + 1
}

fn load_input(name: &str) -> Vec<Vec<char>> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut input: Vec<Vec<char>> = reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();
    input[0][1] = '#';
    input
}

#[derive(Debug, Clone)]
struct Ele {
    x: usize,
    y: usize,
    cur_dist: usize,
    visited: HashSet<(usize, usize)>,
}

impl Ele {
    fn new(x: usize, y: usize) -> Self {
        let mut visited = HashSet::new();
        visited.insert((x, y));
        Self {
            x,
            y,
            cur_dist: 0,
            visited,
        }
    }

    fn new_from(other: &Self, x: usize, y: usize) -> Self {
        let mut visited = other.visited.clone();
        visited.insert((x, y));
        Self {
            x,
            y,
            cur_dist: other.cur_dist + 1,
            visited,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = load_input("example");
        assert_eq!(crate::part_1(&input), 94);
    }

    #[test]
    fn part_2() {
        let input = load_input("example");
        assert_eq!(crate::part_2(&input), 154);
    }
}
