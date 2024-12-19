use std::fmt::Display;
use std::io::BufRead;

pub mod cursor;
pub mod math;

pub type Grid<T> = Vec<Vec<T>>;

/// Struct that helps creating a grid with a border.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct GridMaker<T> {
    inner: Grid<T>,
    border: Option<T>,
}

impl<T> GridMaker<T>
where
    T: Clone,
{
    pub fn new(border: Option<T>) -> Self {
        let inner = match border {
            Some(_) => vec![vec![]],
            None => vec![],
        };
        Self { inner, border }
    }

    /// Create a new empty grid of the given size.
    ///
    /// The `width` and `heigh` are for the empty space so the final `Grid` will be `width + 2` x
    /// `height + 2` if a border is given. `filler` = what to fill the empty grid with.
    pub fn new_empty(border: Option<T>, filler: T, width: usize, height: usize) -> Grid<T> {
        let mut grid = Self::new(border);
        for _ in 0..height {
            grid.push(vec![filler.clone(); width]);
        }
        grid.finish()
    }

    /// Push a new line and insert a T at start and end.
    pub fn push<I>(&mut self, value: I)
    where
        I: IntoIterator<Item = T>,
    {
        let border = match &self.border {
            Some(b) => b,
            None => {
                self.inner.push(value.into_iter().collect());
                return;
            }
        };
        let mut cur = vec![border.clone()];
        cur.extend(value);
        cur.push(border.clone());
        self.inner.push(cur);
    }

    /// Finish creating the grid by adding lines of T at start and end.
    pub fn finish(mut self) -> Grid<T> {
        let border = match &self.border {
            Some(b) => b,
            None => return self.inner,
        };
        if self.inner.len() == 1 {
            return vec![];
        }
        self.inner[0] = vec![border.clone(); self.inner[1].len()];
        self.inner.push(vec![border.clone(); self.inner[1].len()]);
        self.inner
    }
}

pub fn print_grid<T>(grid: &[Vec<T>], border: GridBorderType)
where
    T: Display,
{
    match border {
        GridBorderType::AddBorder(border) => {
            let width = grid[0].len() + 2;
            for _ in 0..width {
                print!("{border}")
            }
            println!();
            for line in grid {
                print!("{border}");
                for tile in line {
                    print!("{tile}");
                }
                print!("{border}");
                println!();
            }
            for _ in 0..width {
                print!("{border}")
            }
            println!();
        }
        GridBorderType::AsIs => {
            for line in grid {
                for ele in line {
                    print!("{ele}")
                }
                println!();
            }
        }
        GridBorderType::RemoveBorder => {
            for y in 1..(grid.len() - 1) {
                for x in 1..(grid[0].len() - 1) {
                    print!("{}", grid[y][x])
                }
                println!();
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum GridBorderType<'a> {
    AddBorder(&'a str),
    AsIs,
    RemoveBorder,
}

impl Default for GridBorderType<'_> {
    fn default() -> Self {
        Self::AsIs
    }
}

pub trait BufReadExt: BufRead {
    /// Returns an iterator over the lines of this reader.
    ///
    /// Same as [`std::io::BufRead::lines()`][std::io::BufRead::lines] but unwraps the result.
    fn lines_unwrap(self) -> LinesUnwrap<Self>
    where
        Self: Sized,
    {
        LinesUnwrap {
            inner: self.lines(),
        }
    }
}

impl<T> BufReadExt for T where T: BufRead {}

#[derive(Debug)]
pub struct LinesUnwrap<B> {
    inner: std::io::Lines<B>,
}

impl<B: std::io::BufRead> Iterator for LinesUnwrap<B> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|r| r.unwrap())
    }
}

#[cfg(test)]
mod grid_maker_tests {
    use super::*;

    #[test]
    fn new_no_border() {
        let new: GridMaker<char> = GridMaker::new(None);
        assert_eq!(
            new,
            GridMaker {
                inner: vec![],
                border: None
            }
        );
        let empty: Vec<Vec<char>> = vec![];
        assert_eq!(new.finish(), empty);
    }

    #[test]
    fn no_border_push() {
        let mut grid = GridMaker::new(None);
        grid.push(['.']);
        assert_eq!(grid.inner, vec![vec!['.']]);
    }

    #[test]
    fn new_border() {
        let new: GridMaker<char> = GridMaker::new(Some('X'));
        assert_eq!(
            new,
            GridMaker {
                inner: vec![vec![]],
                border: Some('X')
            }
        );

        let empty: Vec<Vec<char>> = vec![];
        assert_eq!(new.finish(), empty);
    }

    #[test]
    fn border_push() {
        let mut grid = GridMaker::new(Some('X'));
        grid.push(['.']);
        #[rustfmt::skip]
        let manual = [
            ['X', 'X', 'X'],
            ['X', '.', 'X'],
            ['X', 'X', 'X'],
        ];
        assert_eq!(grid.inner, vec![vec![], vec!['X', '.', 'X']]);
        assert_eq!(grid.finish(), manual);
    }

    #[test]
    fn new_empty_no_border() {
        let new_empty = GridMaker::new_empty(None, '.', 10, 10);
        assert_eq!(new_empty.len(), 10);
        assert_eq!(new_empty[0].len(), 10);
        assert_eq!(new_empty, vec![vec!['.'; 10]; 10]);
    }

    #[test]
    fn new_empty_border() {
        let new_empty = GridMaker::new_empty(Some('X'), '.', 10, 10);
        let manual = [
            ['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X'],
            ['X', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'X'],
            ['X', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'X'],
            ['X', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'X'],
            ['X', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'X'],
            ['X', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'X'],
            ['X', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'X'],
            ['X', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'X'],
            ['X', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'X'],
            ['X', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'X'],
            ['X', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'X'],
            ['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X'],
        ];
        assert_eq!(new_empty.len(), 12);
        assert_eq!(new_empty[0].len(), 12);
        assert_eq!(new_empty, manual);
    }
}
