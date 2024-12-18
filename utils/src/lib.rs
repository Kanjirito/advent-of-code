use std::fmt::Display;
use std::io::BufRead;

/// \[RIGHT, DOWN, LEFT, UP\]
pub const DIAGONAL: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

/// Clockwise starting from right
pub const AROUND: [(isize, isize); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

pub type Grid<T> = Vec<Vec<T>>;

/// Struct that helps creating a grid with a border.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct GridMaker<T> {
    inner: Grid<T>,
    border: T,
}

impl<T> GridMaker<T>
where
    T: Clone,
{
    pub fn new(border: T) -> Self {
        Self {
            inner: vec![Vec::new()],
            border,
        }
    }

    /// Create a new empty grid of the given size.
    ///
    /// The `width` and `heigh` are for the empty space so the final `Grid` will be `width + 2` x
    /// `height + 2`. `empty` = what to fill the empty grid with.
    pub fn new_empty(border: T, empty: T, width: usize, height: usize) -> Grid<T> {
        let mut grid = Self::new(border);
        for _ in 0..height {
            grid.push(vec![empty.clone(); width]);
        }
        grid.finish()
    }

    /// Push a new line and insert a T at start and end.
    pub fn push<I>(&mut self, value: I)
    where
        I: IntoIterator<Item = T>,
    {
        let mut cur = vec![self.border.clone()];
        cur.extend(value);
        cur.push(self.border.clone());
        self.inner.push(cur);
    }

    /// Finish creating the grid by adding lines of T at start and end.
    pub fn finish(mut self) -> Grid<T> {
        self.inner[0] = vec![self.border.clone(); self.inner[1].len()];
        self.inner
            .push(vec![self.border.clone(); self.inner[1].len()]);
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

pub fn divmod<T>(first: T, second: T) -> (T, T)
where
    T: std::ops::Div<Output = T> + std::ops::Rem<Output = T> + Copy,
{
    (first / second, first % second)
}

/// Greatest common divisor
///
/// A number that both of the arguments can be divided by.
pub fn gcd(first: usize, second: usize) -> usize {
    let (bigger, smaller) = if first >= second {
        (first, second)
    } else {
        (second, first)
    };
    let rem = bigger % smaller;
    if rem == 0 {
        smaller
    } else {
        gcd(smaller, rem)
    }
}

/// Least common multiple
///
/// A number that can be divided by both of the arguments.
pub fn lcm(first: usize, second: usize) -> usize {
    (first * second) / gcd(first, second)
}
