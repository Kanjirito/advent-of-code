use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

type Cords = (usize, usize);
const NEIGHBOURS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn main() {
    let mut input = load_input();
    let l_1 = input.len() - 2;
    let l_2 = input[0].len() - 2;
    println!("{}", dijkstra(&mut input, (1, 1), (l_1, l_2)));
}

fn dijkstra(graph: &mut [Vec<Field>], start: Cords, target: Cords) -> usize {
    let mut stack: HashSet<Cords> = HashSet::new();
    for (y, row) in graph.iter().enumerate() {
        for (x, field) in row.iter().enumerate() {
            if let Field::Point { .. } = field {
                stack.insert((y, x));
            }
        }
    }
    match graph[start.0][start.1] {
        Field::Border => unreachable!(),
        Field::Point {
            ref mut shortest_counter,
            ..
        } => {
            *shortest_counter = 0;
        }
    }

    while !stack.is_empty() {
        let current_cords = *stack
            .iter()
            .min_by_key(|(y, x)| -> usize {
                match graph[*y][*x] {
                    Field::Border => unreachable!(),
                    Field::Point {
                        cost: _,
                        shortest_counter,
                        prev_node: _,
                    } => shortest_counter,
                }
            })
            .unwrap();
        stack.remove(&current_cords);
        if current_cords == target {
            break;
        }
        let current_field = graph[current_cords.0][current_cords.1];
        for (y_offset, x_offset) in NEIGHBOURS {
            let new_y = (current_cords.0 as isize + y_offset) as usize;
            let new_x = (current_cords.1 as isize + x_offset) as usize;
            let new_cords = (new_y, new_x);
            if !stack.contains(&new_cords) {
                continue;
            };
            match graph[new_y][new_x] {
                Field::Border => continue,
                Field::Point {
                    cost,
                    ref mut shortest_counter,
                    ref mut prev_node,
                } => {
                    let new_distance = cost
                        + match current_field {
                            Field::Border => unreachable!(),
                            Field::Point {
                                shortest_counter, ..
                            } => shortest_counter,
                        };
                    if new_distance < *shortest_counter {
                        *shortest_counter = new_distance;
                        *prev_node = Some(current_cords);
                    }
                }
            }
        }
    }
    match graph[target.0][target.1] {
        Field::Border => unreachable!(),
        Field::Point {
            shortest_counter, ..
        } => shortest_counter,
    }
}

fn load_input() -> Vec<Vec<Field>> {
    let file = File::open("input").expect("No input file found");
    let reader = BufReader::new(file);
    let mut grid: Vec<Vec<Field>> = Vec::new();
    for line in reader.lines() {
        grid.push(
            line.unwrap()
                .split("")
                .map(|x| x.parse::<Field>().unwrap())
                .collect(),
        )
    }
    let l = grid[0].len();
    grid.push(vec![Field::Border; l]);
    grid.insert(0, vec![Field::Border; l]);
    grid
}

#[derive(Debug, Clone, Copy)]
enum Field {
    Border,
    Point {
        cost: usize,
        shortest_counter: usize,
        prev_node: Option<Cords>,
    },
}

impl FromStr for Field {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.parse::<usize>() {
            Ok(num) => Self::Point {
                cost: num,
                shortest_counter: usize::MAX,
                prev_node: None,
            },
            Err(_) => Self::Border,
        })
    }
}
