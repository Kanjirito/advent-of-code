use std::collections::{HashMap, HashSet};
use std::io::BufReader;
use std::{
    fmt::{Display, Formatter, Result},
    fs::File,
};

use utils::{cursor::*, math::manhattan_distance, BufReadExt, Grid};

fn main() {
    let (grid, start) = load_input("input");
    println!("Solution for part 1: {}", solve(&grid, start, 100, 2));
    println!("Solution for part 1: {}", solve(&grid, start, 100, 20));
}

fn solve(grid: &Grid<Tile>, start: (usize, usize), modif: usize, cheat_amount: usize) -> usize {
    let cursor = Cursor::new(start.0, start.1, &Direction::CARDINAL);
    find_cheats(grid, &find_path(grid, cursor), cheat_amount, modif)
}

fn find_path(grid: &Grid<Tile>, start: Cursor) -> HashMap<(usize, usize), usize> {
    let mut path = vec![start.get_coords()];
    let mut seen = HashSet::new();
    seen.insert(start.get_coords());
    let mut cur = start;

    loop {
        if *cur.index_grid(grid) == Tile::End {
            break;
        }
        for next in cur.get_moves_iter(|t| t.walkable(), grid) {
            if seen.contains(&next.get_coords()) {
                continue;
            }
            path.push(next.get_coords());
            seen.insert(next.get_coords());
            cur = next;
            break;
        }
    }
    let distance = path.len() - 1;
    HashMap::from_iter(
        path.into_iter()
            .enumerate()
            .map(|(i, (x, y))| ((x, y), distance - i)),
    )
}

fn find_cheats(
    grid: &Grid<Tile>,
    distances: &HashMap<(usize, usize), usize>,
    cheat: usize,
    to_beat: usize,
) -> usize {
    let mut solutions = 0;
    for (&(cur_x, cur_y), &cur_distance) in distances {
        for (other_x, other_y, manhat_distance) in
            get_all_in_range(cheat, grid, Cursor::new(cur_x, cur_y, &Direction::CARDINAL))
        {
            let other_distance = distances[&(other_x, other_y)];
            if other_distance >= cur_distance || cur_distance - manhat_distance == other_distance {
                continue;
            } else if cur_distance - manhat_distance - other_distance >= to_beat {
                solutions += 1;
            }
        }
    }
    solutions
}

fn get_all_in_range(
    distance: usize,
    grid: &Grid<Tile>,
    cursor: Cursor,
) -> Vec<(usize, usize, usize)> {
    let start = cursor.get_coords();
    let mut results = vec![];
    let mut seen = HashSet::new();
    seen.insert(start);
    let mut q = vec![cursor];

    while let Some(cur) = q.pop() {
        for next in cur.get_moves_iter(|_| true, grid) {
            if seen.contains(&next.get_coords()) {
                continue;
            }
            let m_distance = manhattan_distance(next.get_coords(), start);
            if m_distance <= distance {
                if next.index_grid(grid).walkable() && m_distance > 1 {
                    results.push((next.get_x(), next.get_y(), m_distance));
                }
                seen.insert(next.get_coords());
                q.push(next);
            }
        }
    }

    results
}

fn load_input(name: &str) -> (Grid<Tile>, (usize, usize)) {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut grid = vec![];
    let mut start = (0, 0);
    for (y, line) in reader.lines_unwrap().enumerate() {
        let mut cur = vec![];
        for (x, c) in line.chars().enumerate() {
            let t = Tile::from(c);
            if let Tile::Start = t {
                start = (x, y);
            }
            cur.push(t);
        }
        grid.push(cur);
    }
    (grid, start)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Ground,
    Wall,
    Start,
    End,
}

impl Tile {
    fn walkable(&self) -> bool {
        match self {
            Tile::Ground | Tile::Start | Tile::End => true,
            Tile::Wall => false,
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Ground,
            '#' => Self::Wall,
            'S' => Self::Start,
            'E' => Self::End,
            _ => unreachable!(),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Ground => '.',
                Tile::Wall => '#',
                Tile::Start => 'S',
                Tile::End => 'E',
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let (grid, start) = load_input("example");
        assert_eq!(solve(&grid, start, 0, 2), 44);
    }

    #[test]
    fn part_2_test() {
        let (grid, start) = load_input("example");
        assert_eq!(solve(&grid, start, 50, 20), 285);
    }
}
