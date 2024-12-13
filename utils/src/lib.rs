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
    T: Copy,
{
    pub fn new(border: T) -> Self {
        Self {
            inner: vec![Vec::new()],
            border,
        }
    }

    /// Push a new line and insert a T at start and end.
    pub fn push<I>(&mut self, value: I)
    where
        I: IntoIterator<Item = T>,
    {
        let mut cur = vec![self.border];
        cur.extend(value);
        cur.push(self.border);
        self.inner.push(cur);
    }

    /// Finish creating the grid by adding lines of T at start and end.
    pub fn finish(mut self) -> Grid<T> {
        self.inner[0] = vec![self.border; self.inner[1].len()];
        self.inner.push(vec![self.border; self.inner[1].len()]);
        self.inner
    }
}

pub fn print_grid<T>(grid: &[Vec<T>])
where
    T: Display,
{
    for line in grid {
        for ele in line {
            print!("{ele}")
        }
        println!();
    }
}

pub fn print_grid_no_border<T>(grid: &[Vec<T>])
where
    T: Display,
{
    for y in 1..(grid.len() - 1) {
        for x in 1..(grid[0].len() - 1) {
            print!("{}", grid[y][x])
        }
        println!();
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
