use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Grid = Vec<Vec<Tile>>;

fn main() {
    let (mut grid, guard) = load_input("input");
    println!("Solution for part 1: {}", part_1(&grid, guard));
    println!("Solution for part 2: {}", part_2(&mut grid, guard));
}

fn part_1(grid: &Grid, mut guard: Guard) -> usize {
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    seen.insert((guard.x, guard.y));

    'outer: loop {
        // (x, y)
        let modif = guard.get_modif();
        let mut x = guard.x;
        let mut y = guard.y;
        loop {
            x = x.saturating_add_signed(modif.0);
            y = y.saturating_add_signed(modif.1);

            match grid[y][x] {
                Tile::Ground => {
                    seen.insert((x, y));
                    guard = Guard {
                        x,
                        y,
                        dire: guard.dire,
                    };
                }
                Tile::Block => {
                    guard.rotate();
                    break;
                }
                Tile::Wall => break 'outer,
            }
        }
    }

    seen.len()
}

fn part_2(grid: &mut Grid, guard: Guard) -> usize {
    let mut possible: HashSet<(usize, usize)> = HashSet::new();
    walk(grid, guard, HashSet::new(), &mut possible, true);
    // Can't place block on start position
    // possible.remove(&(guard.x, guard.y));
    possible.len()
}

fn walk(
    grid: &mut Grid,
    mut guard: Guard,
    mut seen: HashSet<Guard>,
    possible: &mut HashSet<(usize, usize)>,
    can_block: bool,
) -> bool {
    loop {
        if seen.contains(&guard) {
            // We've already been here so it's a loop
            return true;
        }
        seen.insert(guard);
        // (x, y)
        let modif = guard.get_modif();
        let x = guard.x.saturating_add_signed(modif.0);
        let y = guard.y.saturating_add_signed(modif.1);

        // Check the next tile
        match grid[y][x] {
            // We are on ground and the next tile is also ground
            Tile::Ground => {
                // first check if inserting a block there would cause a loop
                if can_block {
                    // Checks if the next tile wasn't visited already, if yes block can't be
                    // placed there
                    let mut tmp = Guard { x, y, dire: 0 };
                    let mut valid = true;
                    for _ in 0..4 {
                        if seen.contains(&tmp) {
                            valid = false;
                            break;
                        }
                        tmp.rotate();
                    }

                    let mut g = guard;
                    g.rotate();
                    // Place the new block
                    grid[y][x] = Tile::Block;
                    if valid && walk(grid, g, seen.clone(), possible, false) {
                        possible.insert((x, y));
                    }
                    // Remove the block
                    grid[y][x] = Tile::Ground;
                }

                // and then just go to that tile
                guard = Guard {
                    x,
                    y,
                    dire: guard.dire,
                };
            }
            // Block ahead so just rotate
            Tile::Block => {
                guard.rotate();
            }
            // Out of bounds so no loop
            Tile::Wall => return false,
        }
    }
}

fn load_input(name: &str) -> (Grid, Guard) {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    // Dummy guard
    let mut guard = Guard::new(0, 0, '^');
    let mut grid = vec![vec![]];
    for (y, line) in reader.lines().map(|l| l.unwrap()).enumerate() {
        let mut cur = vec![Tile::Wall];
        for (x, c) in line.chars().enumerate() {
            match Tile::try_from(c) {
                Ok(t) => {
                    cur.push(t);
                }
                Err(_) => {
                    guard = Guard::new(x + 1, y + 1, c);
                    cur.push(Tile::Ground);
                }
            }
        }
        cur.push(Tile::Wall);
        grid.push(cur);
    }
    grid[0] = vec![Tile::Wall; grid[1].len()];
    grid.push(vec![Tile::Wall; grid[1].len()]);
    (grid, guard)
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Guard {
    x: usize,
    y: usize,
    dire: u8,
}

impl Guard {
    fn new(x: usize, y: usize, dire: char) -> Self {
        let dire = match dire {
            '^' => 0,
            '>' => 1,
            'v' => 2,
            '<' => 3,
            _ => unreachable!(),
        };

        Self { x, y, dire }
    }

    fn rotate(&mut self) {
        self.dire = (self.dire + 1) % 4;
    }

    fn get_modif(&self) -> (isize, isize) {
        match self.dire {
            0 => (0, -1),
            1 => (1, 0),
            2 => (0, 1),
            3 => (-1, 0),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Ground,
    Block,
    Wall,
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Ground),
            '#' => Ok(Self::Block),
            _ => Err(()),
        }
    }
}

#[allow(dead_code)]
fn print_grid(grid: &Grid) {
    for line in grid {
        for tile in line {
            match tile {
                Tile::Ground => print!("."),
                Tile::Block => print!("#"),
                Tile::Wall => print!("X"),
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
        let (grid, guard) = load_input("example");
        assert_eq!(part_1(&grid, guard), 41);
    }

    #[test]
    fn part_2_test() {
        let (mut grid, guard) = load_input("example");
        assert_eq!(part_2(&mut grid, guard), 6);
    }

    #[test]
    fn part_2_test_custom() {
        let (mut grid, guard) = load_input("example2");
        assert_eq!(part_2(&mut grid, guard), 1);
    }
}
