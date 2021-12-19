use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

type Cords = (usize, usize);
const NEIGHBOURS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn main() {
    let input = load_input();
    let original_len_1 = (input.len() - 2) / 5;
    let original_len_2 = (input[0].len() - 2) / 5;
    println!(
        "Solution for part 1: {}",
        dijkstra(&input, (1, 1), (original_len_1, original_len_2))
    );
    let extended_len_1 = input.len() - 2;
    let extended_len_2 = input[0].len() - 2;
    println!(
        "Solution for part 2: {}",
        dijkstra(&input, (1, 1), (extended_len_1, extended_len_2))
    );
}

fn dijkstra(grid: &[Vec<Field>], start: Cords, target: Cords) -> usize {
    let mut distance: Vec<Vec<usize>> = vec![vec![usize::MAX; grid[0].len()]; grid.len()];
    let mut heap = BinaryHeap::new();
    match grid[start.0][start.1] {
        // Start position will never be border
        Field::Border => unreachable!(),
        Field::Point { .. } => heap.push(DijkstraElement {
            current_cost: 0,
            position: start,
        }),
    }
    distance[start.0][start.1] = 0;
    while let Some(DijkstraElement {
        current_cost,
        position,
    }) = heap.pop()
    {
        if position == target {
            break;
        }
        if current_cost > distance[position.0][position.1] {
            continue;
        }
        for (y_offset, x_offset) in NEIGHBOURS {
            let new_y = (position.0 as isize + y_offset) as usize;
            let new_x = (position.1 as isize + x_offset) as usize;
            match grid[new_y][new_x] {
                Field::Border => continue,
                Field::Point(next_cost) => {
                    let next_element = DijkstraElement {
                        current_cost: next_cost + current_cost,
                        position: (new_y, new_x),
                    };
                    if next_element.current_cost < distance[new_y][new_x] {
                        distance[new_y][new_x] = next_element.current_cost;
                        heap.push(next_element)
                    }
                }
            }
        }
    }
    distance[target.0][target.1]
}

fn load_input() -> Vec<Vec<Field>> {
    let mut file = File::open("input").expect("No input file found");
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let mut grid: Vec<Vec<Field>> = Vec::new();
    for down in 0..5 {
        for line in input.split('\n').filter(|x| !x.is_empty()) {
            let mut row: Vec<Field> = vec![Field::Border];
            for right in 0..5 {
                for part in line.split("").filter(|x| !x.is_empty()) {
                    let mut num = (part.parse::<usize>().unwrap() + down + right) % 9;
                    if num == 0 {
                        num = 9;
                    };
                    row.push(Field::new_point(num));
                }
            }
            row.push(Field::Border);
            grid.push(row);
        }
    }
    let l = grid[0].len();
    grid.push(vec![Field::Border; l]);
    grid.insert(0, vec![Field::Border; l]);
    grid
}

#[derive(Debug, Clone, Copy)]
enum Field {
    Border,
    Point(usize),
}

impl Field {
    fn new_point(num: usize) -> Self {
        Self::Point(num)
    }
}

impl FromStr for Field {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.parse::<usize>() {
            Ok(num) => Self::Point(num),
            Err(_) => Self::Border,
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct DijkstraElement {
    current_cost: usize,
    position: Cords,
}

impl Ord for DijkstraElement {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .current_cost
            .cmp(&self.current_cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for DijkstraElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
