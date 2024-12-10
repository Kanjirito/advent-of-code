use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input();
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", count_overlaps(&input));
}

fn part_1(input: &[Line]) -> usize {
    let straight_lines: Vec<Line> = input
        .iter()
        .filter(|x| match x.orientation {
            Orientation::Diagonal => false,
            Orientation::Horizontal | Orientation::Vertical => true,
        })
        .copied()
        .collect();
    count_overlaps(&straight_lines)
}

fn count_overlaps(lines: &[Line]) -> usize {
    let mut point_counter: HashMap<Point, usize> = HashMap::new();
    for point in lines.iter().flat_map(|x| x.points()) {
        let count = point_counter.entry(point).or_insert(0);
        *count += 1
    }
    point_counter.iter().filter(|x| x.1 >= &2).count()
}

#[derive(Debug, Clone, Copy)]
enum Orientation {
    Horizontal,
    Vertical,
    Diagonal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
struct Line {
    start: Point,
    end: Point,
    orientation: Orientation,
}

impl Line {
    fn new(start: Point, end: Point) -> Self {
        let orientation: Orientation;
        if start.x == end.x {
            orientation = Orientation::Vertical;
        } else if start.y == end.y {
            orientation = Orientation::Horizontal;
        } else {
            orientation = Orientation::Diagonal;
        }
        Self {
            start,
            end,
            orientation,
        }
    }

    fn points(&self) -> LineIter {
        LineIter::new(self.start, self.end)
    }
}

struct LineIter {
    current: Point,
    end: Point,
    modifier: (isize, isize),
    first_run: bool,
}

impl LineIter {
    fn new(start: Point, end: Point) -> Self {
        let x_modif: isize = match start.x.cmp(&end.x) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };
        let y_modif: isize = match start.y.cmp(&end.y) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };

        Self {
            current: start,
            end,
            modifier: (x_modif, y_modif),
            first_run: true,
        }
    }
}

impl Iterator for LineIter {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            return None;
        } else if self.first_run {
            self.first_run = false;
            return Some(self.current);
        }
        let new_point = Point::new(
            self.current.x + self.modifier.0,
            self.current.y + self.modifier.1,
        );
        self.current = new_point;
        Some(new_point)
    }
}

fn load_input() -> Vec<Line> {
    let file = File::open("input").expect("No input file found");
    let reader = BufReader::new(file);
    let mut lines: Vec<Line> = Vec::new();
    for line in reader.lines() {
        let cords: Vec<isize> = line
            .unwrap()
            .split(" -> ")
            .flat_map(|x| x.split(','))
            .map(|z| z.parse().unwrap())
            .collect();
        lines.push(Line::new(
            Point::new(cords[0], cords[1]),
            Point::new(cords[2], cords[3]),
        ))
    }
    lines
}
