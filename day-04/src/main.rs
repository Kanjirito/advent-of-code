use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let (guesses, mut boards) = load_input();
    'outer: for guess in guesses {
        for board in &mut boards {
            if board.check_guess(guess) && board.check_if_won() {
                println!("{}", board.count_unhit() * board.last_hit.unwrap());
                break 'outer;
            }
        }
    }
}

#[derive(Debug)]
struct Field {
    value: usize,
    hit: bool,
}

impl Field {
    fn new(value: usize) -> Self {
        Self { value, hit: false }
    }

    fn is_hit(&self) -> bool {
        self.hit
    }
}

#[derive(Debug)]
struct BingoBoard {
    rows: Vec<Vec<Field>>,
    last_hit: Option<usize>,
}

impl BingoBoard {
    fn new(rows: Vec<Vec<Field>>) -> Self {
        Self {
            rows,
            last_hit: None,
        }
    }

    fn check_guess(&mut self, guess: usize) -> bool {
        let mut hit = false;
        for row in &mut self.rows {
            for field in row {
                if field.value == guess {
                    field.hit = true;
                    self.last_hit = Some(guess);
                    hit = true;
                }
            }
        }
        hit
    }

    fn check_if_won(&self) -> bool {
        for row in &self.rows {
            if row.iter().all(|item| item.is_hit()) {
                return true;
            }
        }
        for n in 0..5 {
            let mut miss = false;
            for row in &self.rows {
                if !row[n].is_hit() {
                    miss = true;
                    break;
                }
            }
            if !miss {
                return true;
            }
        }
        false
    }

    fn count_unhit(&self) -> usize {
        let mut counter: usize = 0;
        for row in &self.rows {
            for field in row {
                if !field.is_hit() {
                    counter += field.value;
                }
            }
        }
        counter
    }
}

fn load_input() -> (Vec<usize>, Vec<BingoBoard>) {
    let file = File::open("input").expect("No input file found");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let guesses: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect();

    // Skip next line because it's empty
    lines.next();
    let mut boards: Vec<BingoBoard> = Vec::new();
    loop {
        let mut rows: Vec<Vec<Field>> = Vec::new();
        for _ in 0..5 {
            let line = lines.next().unwrap().unwrap();
            rows.push(
                line.split_whitespace()
                    .map(|n| {
                        let number: usize = n.parse().unwrap();
                        Field::new(number)
                    })
                    .collect(),
            )
        }
        boards.push(BingoBoard::new(rows));
        if lines.next().is_none() {
            break;
        }
    }
    (guesses, boards)
}
