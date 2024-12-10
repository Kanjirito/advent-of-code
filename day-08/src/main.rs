use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input();
    let matrix = get_visibility(&input);
    let (p_1, p_2) = solve(&input, &matrix);
    println!("Part 1: {}", p_1);
    println!("Part 2: {}", p_2);
}

fn solve(input: &[Vec<usize>], matrix: &[Vec<bool>]) -> (usize, usize) {
    let mut counter: usize = 0;
    let mut cur_high = 0;
    for (x, row) in matrix.iter().enumerate() {
        for (y, tree) in row.iter().enumerate() {
            if *tree {
                counter += 1;
                cur_high = cur_high.max(get_score(x, y, input))
            }
        }
    }
    (counter, cur_high)
}

/// Get the visibility score for tree at (x, y)
///
/// Simply walks in each direction from the tree and counts trees until it finds one that isn't smaller.
fn get_score(x: usize, y: usize, input: &[Vec<usize>]) -> usize {
    let mut counter = vec![0; 4];
    let cur_height = input[x][y];
    let mut i = x;
    let mut j = y;

    while let (_, false) = i.overflowing_sub(1) {
        i -= 1;
        counter[0] += 1;
        if input[i][j] >= cur_height {
            break;
        }
    }

    i = x;
    while i < input.len() - 1 {
        i += 1;
        counter[1] += 1;
        if input[i][j] >= cur_height {
            break;
        }
    }

    i = x;
    while let (_, false) = j.overflowing_sub(1) {
        j -= 1;
        counter[2] += 1;
        if input[i][j] >= cur_height {
            break;
        }
    }

    j = y;
    while j < input[0].len() - 1 {
        j += 1;
        counter[3] += 1;
        if input[i][j] >= cur_height {
            break;
        }
    }
    counter.iter().product()
}

/// Finds the visible trees
///
/// Returns a 2D matrix of booleans that shows if a tree at (x, y) is visible from the side.
/// Iterates over the trees 4 times from different directions and keeps track of the tallest seen tree from that direction.
/// If there was no taller tree so far than the current one it means it's visible from that side. Breaks earlier if the tallest
/// tree was 9 tall because trees can't be taller.
fn get_visibility(input: &[Vec<usize>]) -> Vec<Vec<bool>> {
    let height = input.len();
    let width = input[0].len();
    let mut matrix: Vec<Vec<bool>> = vec![vec![false; width]; height];

    // Rows left to right
    for x in 0..height {
        let mut cur_tallest = input[x][0];
        matrix[x][0] = true;
        for y in 1..width {
            let cur_height = input[x][y];
            if cur_tallest == 9 {
                break;
            } else if cur_height > cur_tallest {
                matrix[x][y] = true;
                cur_tallest = cur_height;
            }
        }
    }

    // Rows right to left
    for x in 0..height {
        let mut cur_tallest = input[x][width - 1];
        matrix[x][width - 1] = true;
        for y in (1..width).rev() {
            let cur_height = input[x][y];
            if cur_tallest == 9 {
                break;
            } else if cur_height > cur_tallest {
                matrix[x][y] = true;
                cur_tallest = cur_height;
            }
        }
    }

    // Columns top to bottom
    for x in 0..width {
        let mut cur_tallest = input[0][x];
        matrix[0][x] = true;
        for y in 1..height {
            let cur_height = input[y][x];
            if cur_tallest == 9 {
                break;
            } else if cur_height > cur_tallest {
                matrix[y][x] = true;
                cur_tallest = cur_height;
            }
        }
    }

    // Columns bottom to top
    for x in 0..width {
        let mut cur_tallest = input[height - 1][x];
        matrix[height - 1][x] = true;
        for y in (1..height).rev() {
            let cur_height = input[y][x];
            if cur_tallest == 9 {
                break;
            } else if cur_height > cur_tallest {
                matrix[y][x] = true;
                cur_tallest = cur_height;
            }
        }
    }
    matrix
}

fn load_input() -> Vec<Vec<usize>> {
    let file = File::open("input").expect("No input file found");
    let reader = BufReader::new(file);
    let mut input: Vec<Vec<usize>> = Vec::new();

    for line in reader.lines().map(|l| l.unwrap()) {
        input.push(
            line.chars()
                .map(|d| d.to_digit(10).expect("Not a number") as usize)
                .collect(),
        );
    }
    input
}
