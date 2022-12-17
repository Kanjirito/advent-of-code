use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let moves = load_input("input");
    let result = solve(moves, 2022, 1000000000000);
    println!("Part 1: {}", result.0);
    println!("Part 2: {}", result.1);
}

fn solve(mut moves: Moves, first_target: i64, second_target: i64) -> (i64, i64) {
    // Initializes the chamber with a floor
    let mut chamber: HashSet<(i64, i64)> = HashSet::new();
    for x in 1..=7 {
        chamber.insert((x, 0));
    }
    let mut shapes = Shapes::new();

    // The height reached for the first target
    let mut first_height = 0;
    // Check if the cycle skip has happened
    let mut before_cycle = true;
    // Keeping track of already visited block and move combinations and their rock count and height
    let mut cache: HashMap<(usize, usize), (i64, i64)> = HashMap::new();
    // Cur highest point
    let mut highest = 0;
    // How much was skipped through the cycles
    let mut height_skipped = 0;
    let mut i: i64 = 0;
    while i < second_target {
        // Stores the index of the current shape and move
        let cur_s = shapes.i;
        let cur_m = moves.i;
        // Simulates a rock drop
        highest = highest.max(drop_rock(&mut chamber, &mut moves, &mut shapes, highest));
        // Keep dropping rocks normally until the first target is reached
        if i == first_target - 1 {
            // First target was reached, value is saved
            first_height = highest;
        } else if i > first_target - 1 && before_cycle {
            // After first target is passed start to look for cycle
            match cache.get(&(cur_s, cur_m)) {
                // Current rock and move combination hasn't been seen yet, keep track of it
                None => {
                    cache.insert((cur_s, cur_m), (i, highest));
                }
                // The combination was seen before
                Some((prev_i, prev_h)) => {
                    // Gets the rock count in a cycle
                    let i_diff = i - prev_i;
                    // Gets the height difference in a cycle
                    let h_diff = highest - prev_h;
                    // How many cycles fit into what's left
                    let cycle_count = (second_target - i) / i_diff;
                    // Skip as many rocks as possible
                    i += i_diff * cycle_count;
                    // Save the height difference from the skip
                    // This needs to be saved on it's own because otherwise the rocks will be dropped from very high up
                    height_skipped = h_diff * cycle_count;
                    // Don't try to skip any more and just simulate the rest
                    before_cycle = false;
                }
            }
        }
        i += 1;
    }
    // Add the skip difference to the current height
    (first_height, highest + height_skipped)
}

fn drop_rock(
    chamber: &mut HashSet<(i64, i64)>,
    moves: &mut Moves,
    shapes: &mut Shapes,
    highest: i64,
) -> i64 {
    // Create a new rock with 3 empty spaces
    let mut rock = shapes.next(highest + 4);
    loop {
        // Move left or right
        rock.move_rock(chamber, moves.next());
        // Move down, if false it means the rock landed on something
        if !rock.move_rock(chamber, Move::Down) {
            // Get the highest point of the rock and return it
            let mut max = 0;
            for pos in rock.fields {
                max = max.max(pos.1);
                chamber.insert(pos);
            }
            return max;
        }
    }
}

fn load_input(name: &str) -> Moves {
    let file = File::open(name).expect("No input file found");
    let reader = BufReader::new(file);
    let moves = reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| c.into())
        .collect();
    Moves { moves, i: 0 }
}

#[derive(Debug, Clone)]
struct Moves {
    moves: Vec<Move>,
    i: usize,
}
impl Moves {
    fn next(&mut self) -> Move {
        let n = self.moves[self.i];
        self.i += 1;
        self.i %= self.moves.len();
        n
    }
}

#[derive(Debug)]
struct Shapes {
    i: usize,
    shapes: Vec<Vec<(i64, i64)>>,
}

impl Shapes {
    fn new() -> Self {
        let i = 0;
        let shapes = vec![
            // Line horizontal
            vec![(3, 0), (4, 0), (5, 0), (6, 0)],
            // Plus
            vec![(3, 1), (4, 0), (4, 2), (5, 1)],
            // L
            vec![(3, 0), (4, 0), (5, 0), (5, 1), (5, 2)],
            // Line vertical
            vec![(3, 0), (3, 1), (3, 2), (3, 3)],
            // Square
            vec![(3, 0), (3, 1), (4, 0), (4, 1)],
        ];
        Self { i, shapes }
    }

    fn next(&mut self, y: i64) -> Shape {
        let mut new_shape = Vec::new();
        for (i, j) in &self.shapes[self.i] {
            new_shape.push((*i, y + j));
        }
        self.i += 1;
        self.i %= self.shapes.len();
        Shape { fields: new_shape }
    }
}
#[derive(Debug)]
struct Shape {
    fields: Vec<(i64, i64)>,
}

impl Shape {
    fn move_rock(&mut self, taken: &HashSet<(i64, i64)>, direction: Move) -> bool {
        let mut new_pos = Vec::new();
        let modif = match direction {
            Move::Left => (-1, 0),
            Move::Right => (1, 0),
            Move::Down => (0, -1),
        };
        for (x, y) in &self.fields {
            let i = x + modif.0;
            let j = y + modif.1;
            if i == 0 || i > 7 || taken.contains(&(i, j)) {
                return false;
            }
            new_pos.push((i, j));
        }
        self.fields = new_pos;
        true
    }
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Left,
    Right,
    Down,
}

impl From<char> for Move {
    fn from(value: char) -> Self {
        match value {
            '>' => Self::Right,
            '<' => Self::Left,
            _ => unreachable!(),
        }
    }
}

#[test]
fn example() {
    let moves = load_input("example");
    let result = solve(moves, 2022, 1000000000000);
    println!("Part 1: {}", result.0);
    assert_eq!(result.0, 3068);
    assert_eq!(result.1, 1514285714288);
}
