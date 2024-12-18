use crate::Grid;

use modifiers::*;

pub mod modifiers {
    /// \[N, E, S, W\]
    pub const CARDINAL_MODIF: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    /// \[NE, SE, SW, NW\]
    pub const DIAGONAL_MODIF: [(isize, isize); 4] = [(1, -1), (1, 1), (-1, 1), (-1, -1)];

    /// Clockwise starting from N
    pub const AROUND_MODIF: [(isize, isize); 8] = [
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ];
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cursor {
    x: usize,
    y: usize,
    cur_dire: usize,
    allowed_dire: Vec<Direction>,
}

impl Cursor {
    pub fn new(x: usize, y: usize, directions: &[Direction]) -> Self {
        Self {
            x,
            y,
            cur_dire: 0,
            allowed_dire: directions.to_vec(),
        }
    }
    pub fn move_forward<F, T>(&mut self, check: F, grid: &Grid<T>) -> MoveResult
    where
        F: Fn(&T) -> bool,
    {
        let modif = AROUND_MODIF[usize::from(self.get_cur_dire())];

        let new_x = match self.x.checked_add_signed(modif.0) {
            Some(x) if x < grid[0].len() => x,
            _ => return MoveResult::OutOfBounds,
        };

        let new_y = match self.y.checked_add_signed(modif.1) {
            Some(y) if y < grid.len() => y,
            _ => return MoveResult::OutOfBounds,
        };

        if check(&grid[new_y][new_x]) {
            self.x = new_x;
            self.y = new_y;
            MoveResult::Ok
        } else {
            MoveResult::CheckFailed
        }
    }

    pub fn turn_right(&mut self) -> Direction {
        self.cur_dire = (self.cur_dire + 1) % self.allowed_dire.len();
        self.get_cur_dire()
    }

    pub fn turn_left(&mut self) -> Direction {
        self.cur_dire = (self.cur_dire.overflowing_sub(1).0) % self.allowed_dire.len();
        self.get_cur_dire()
    }

    pub fn get_cur_dire(&self) -> Direction {
        self.allowed_dire[self.cur_dire]
    }

    pub fn set_cur_dire(&mut self, target: usize) {
        // Panics if out of bounds
        let _ = self.allowed_dire[target];
        self.cur_dire = target;
    }

    pub fn get_allowed_dires(&self) -> &[Direction] {
        &self.allowed_dire
    }

    pub fn get_coords(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn get_unique_pos(&self) -> (usize, usize, Direction) {
        (self.x, self.y, self.get_cur_dire())
    }

    pub fn get_moves_iter<'grid, 'cursor, F, T>(
        &'cursor self,
        check: F,
        grid: &'grid Grid<T>,
    ) -> Moves<'grid, 'cursor, T, F>
    where
        F: Fn(&T) -> bool,
    {
        Moves::new(self, grid, check)
    }

    pub fn get_all_moves<T, F>(&self, check: F, grid: &Grid<T>) -> Vec<Self>
    where
        F: Fn(&T) -> bool,
    {
        Moves::new(self, grid, check).collect()
    }
}

#[derive(Debug)]
pub struct Moves<'grid, 'cursor, T, F> {
    start_cursor: &'cursor Cursor,
    grid: &'grid Grid<T>,
    check: F,
    moves: usize,
}

impl<'grid, 'cursor, T, F> Moves<'grid, 'cursor, T, F>
where
    F: Fn(&T) -> bool,
{
    fn new(start: &'cursor Cursor, grid: &'grid Grid<T>, check: F) -> Moves<'grid, 'cursor, T, F> {
        Self {
            start_cursor: start,
            grid,
            check,
            moves: 0,
        }
    }
}

impl<T, F> Iterator for Moves<'_, '_, T, F>
where
    F: Fn(&T) -> bool,
{
    type Item = Cursor;

    fn next(&mut self) -> Option<Self::Item> {
        while self.moves < self.start_cursor.allowed_dire.len() {
            let mut new_cursor = Cursor::clone(&self.start_cursor.clone());
            for _ in 0..self.moves {
                new_cursor.turn_right();
            }
            self.moves += 1;
            if let MoveResult::Ok = new_cursor.move_forward(&self.check, self.grid) {
                return Some(new_cursor);
            }
        }

        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoveResult {
    Ok,
    OutOfBounds,
    CheckFailed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    pub const CARDINAL: [Self; 4] = [Self::N, Self::E, Self::S, Self::W];
    pub const DIAGONAL: [Self; 4] = [Self::NE, Self::SE, Self::SW, Self::NW];
    pub const AROUND: [Self; 8] = [
        Self::N,
        Self::NE,
        Self::E,
        Self::SE,
        Self::S,
        Self::SW,
        Self::W,
        Self::NW,
    ];
}

impl From<Direction> for usize {
    fn from(value: Direction) -> Self {
        match value {
            Direction::N => 0,
            Direction::NE => 1,
            Direction::E => 2,
            Direction::SE => 3,
            Direction::S => 4,
            Direction::SW => 5,
            Direction::W => 6,
            Direction::NW => 7,
        }
    }
}
