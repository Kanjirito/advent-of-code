use std::fmt;
use std::io::BufReader;
use std::{collections::HashSet, fs::File};

use utils::{BufReadExt, Grid, DIAGONAL};

fn main() {
    let (state, moves) = load_input("input");
    println!("Solution for part 1: {}", part_1(state.clone(), &moves));
    println!("Solution for part 2: {}", part_2(state, &moves));
}

fn part_1(mut state: State, moves: &[Move]) -> usize {
    for &cur_move in moves {
        state.make_move(cur_move);
    }
    count_score(&state.grid)
}

fn part_2(mut state: State, moves: &[Move]) -> usize {
    state.double();
    for &cur_move in moves {
        state.make_move_2(cur_move);
    }
    count_score(&state.grid)
}

fn count_score(grid: &Grid<Tile>) -> usize {
    let mut counter = 0;
    for (y, line) in grid.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            match tile {
                Tile::BoxStart | Tile::Box => {
                    counter += (y * 100) + x;
                }
                _ => {}
            }
        }
    }
    counter
}

fn load_input(name: &str) -> (State, Vec<Move>) {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut lines = reader.lines_unwrap().enumerate();

    let mut grid: Grid<Tile> = vec![];
    let mut robot: (usize, usize) = (0, 0);

    for (y, line) in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let mut cur = vec![];
        for (x, tile) in line.chars().map(|c| c.try_into().unwrap()).enumerate() {
            if tile == Tile::Robot {
                robot = (x, y);
                // tile = Tile::Ground;
            }
            cur.push(tile);
        }
        grid.push(cur);
    }

    let moves: Vec<Move> = lines
        .flat_map(|(_, l)| l.chars().map(|c| c.try_into().unwrap()).collect::<Vec<_>>())
        .collect();

    (
        State {
            grid,
            robot_x: robot.0,
            robot_y: robot.1,
        },
        moves,
    )
}

#[derive(Debug, Clone)]
struct State {
    grid: Grid<Tile>,
    robot_x: usize,
    robot_y: usize,
}

impl State {
    fn make_move(&mut self, cur_move: Move) {
        let (mod_x, mod_y) = cur_move.get_modif();
        let target_x = self.robot_x.saturating_add_signed(mod_x);
        let target_y = self.robot_y.saturating_add_signed(mod_y);

        let mut cur_x = self.robot_x;
        let mut cur_y = self.robot_y;

        loop {
            cur_x = cur_x.saturating_add_signed(mod_x);
            cur_y = cur_y.saturating_add_signed(mod_y);
            match self.grid[cur_y][cur_x] {
                Tile::Box => {
                    // Skip over the box
                    continue;
                }
                Tile::Ground | Tile::Robot => {
                    // Push all the boxes and move the robot 1 tile
                    self.grid[cur_y][cur_x] = Tile::Box;
                    self.grid[target_y][target_x] = Tile::Ground;
                    self.robot_x = target_x;
                    self.robot_y = target_y;
                    return;
                }
                Tile::Wall => {
                    // Can't move so just leave
                    return;
                }
                Tile::BoxStart | Tile::BoxEnd => unreachable!(),
            }
        }
    }

    fn make_move_2(&mut self, cur_move: Move) {
        let (before_x, before_y) = (self.robot_x, self.robot_y);
        match cur_move {
            Move::Up => self.move_vertical(true),
            Move::Down => self.move_vertical(false),
            Move::Right => self.move_horizontal(true),
            Move::Left => self.move_horizontal(false),
        }
        self.grid[before_y][before_x] = Tile::Ground;
        self.grid[self.robot_y][self.robot_x] = Tile::Robot;
    }

    fn move_horizontal(&mut self, right: bool) {
        let mod_x: isize = if right { 1 } else { -1 };

        let target_x = self.robot_x.saturating_add_signed(mod_x);

        let mut cur_x = self.robot_x;
        let cur_y = self.robot_y;

        loop {
            cur_x = cur_x.saturating_add_signed(mod_x);
            match self.grid[cur_y][cur_x] {
                Tile::BoxStart | Tile::BoxEnd => {
                    continue;
                }
                Tile::Ground => {
                    // Push all the boxes and move the robot 1 tile
                    if right {
                        self.grid[cur_y][target_x..=cur_x].rotate_right(1);
                    } else {
                        self.grid[cur_y][cur_x..=target_x].rotate_left(1);
                    }
                    self.robot_x = target_x;
                    return;
                }
                Tile::Wall => {
                    // Can't move so just leave
                    return;
                }
                Tile::Box | Tile::Robot => {
                    unreachable!()
                }
            }
        }
    }

    fn move_vertical(&mut self, up: bool) {
        let mod_y: isize = if up { -1 } else { 1 };
        let mut to_move: Vec<(usize, usize, Tile)> = Vec::new();
        if self.check_boxes(self.robot_x, self.robot_y, mod_y, &mut to_move) {
            self.robot_y = self.robot_y.saturating_add_signed(mod_y);
            let mut deduped: HashSet<(usize, usize, Tile)> =
                HashSet::from_iter(to_move.iter().cloned());
            for (move_x, move_y, tile) in to_move {
                if !deduped.remove(&(move_x, move_y, tile)) {
                    continue;
                };
                let target_y = move_y.saturating_add_signed(mod_y);
                self.grid[move_y][move_x] = Tile::Ground;
                match tile {
                    Tile::BoxStart => {
                        self.grid[target_y][move_x] = Tile::BoxStart;
                    }
                    Tile::BoxEnd => self.grid[target_y][move_x] = Tile::BoxEnd,
                    _ => {}
                }
            }
        }
    }

    fn check_boxes(
        &mut self,
        cur_x: usize,
        cur_y: usize,
        mod_y: isize,
        to_move: &mut Vec<(usize, usize, Tile)>,
    ) -> bool {
        let next_y = cur_y.saturating_add_signed(mod_y);

        match self.grid[cur_y][cur_x] {
            Tile::BoxStart => {
                if self.check_boxes(cur_x, next_y, mod_y, to_move)
                    && self.check_boxes(cur_x + 1, next_y, mod_y, to_move)
                {
                    to_move.push((cur_x, cur_y, Tile::BoxStart));
                    to_move.push((cur_x + 1, cur_y, Tile::BoxEnd));
                    true
                } else {
                    false
                }
            }
            Tile::BoxEnd => {
                if self.check_boxes(cur_x, next_y, mod_y, to_move)
                    && self.check_boxes(cur_x - 1, next_y, mod_y, to_move)
                {
                    to_move.push((cur_x - 1, cur_y, Tile::BoxStart));
                    to_move.push((cur_x, cur_y, Tile::BoxEnd));
                    true
                } else {
                    false
                }
            }
            Tile::Ground => true,
            Tile::Wall => false,
            Tile::Robot => self.check_boxes(cur_x, next_y, mod_y, to_move),
            Tile::Box => unreachable!(),
        }
    }

    fn double(&mut self) {
        let mut new_grid = Vec::with_capacity(self.grid.len());

        for line in &self.grid {
            let mut new_line = Vec::with_capacity(self.grid[0].len() * 2);
            for t in line {
                match t {
                    Tile::Box => {
                        new_line.push(Tile::BoxStart);
                        new_line.push(Tile::BoxEnd);
                    }
                    Tile::Ground => new_line.extend_from_slice(&[Tile::Ground, Tile::Ground]),
                    Tile::Wall => new_line.extend_from_slice(&[Tile::Wall, Tile::Wall]),
                    Tile::Robot => new_line.extend_from_slice(&[Tile::Robot, Tile::Ground]),
                    Tile::BoxStart | Tile::BoxEnd => unreachable!(),
                }
            }
            new_grid.push(new_line);
        }
        self.robot_x *= 2;
        self.grid = new_grid;
    }
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Up,
    Right,
    Down,
    Left,
}

impl Move {
    fn get_modif(&self) -> (isize, isize) {
        match self {
            Move::Up => DIAGONAL[3],
            Move::Right => DIAGONAL[0],
            Move::Down => DIAGONAL[1],
            Move::Left => DIAGONAL[2],
        }
    }
}

impl TryFrom<char> for Move {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Self::Up),
            '>' => Ok(Self::Right),
            'v' => Ok(Self::Down),
            '<' => Ok(Self::Left),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Box,
    BoxStart,
    BoxEnd,
    Ground,
    Robot,
    Wall,
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'O' => Ok(Self::Box),
            '.' => Ok(Self::Ground),
            '@' => Ok(Self::Robot),
            '#' => Ok(Self::Wall),
            '[' => Ok(Self::BoxStart),
            ']' => Ok(Self::BoxEnd),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Box => 'O',
                Self::Ground => '.',
                Self::Robot => '@',
                Self::Wall => '#',
                Self::BoxStart => '[',
                Self::BoxEnd => ']',
            }
        )
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test_short() {
        let (state, moves) = load_input("example");
        assert_eq!(part_1(state, &moves), 2028);
    }

    #[test]
    fn part_1_test_long() {
        let (state, moves) = load_input("example2");
        assert_eq!(part_1(state, &moves), 10092);
    }

    #[test]
    fn part_2_test() {
        let (state, moves) = load_input("example2");
        assert_eq!(part_2(state, &moves), 9021);
    }
}
