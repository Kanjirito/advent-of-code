use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

fn main() {
    let input = load_input();
    let mut part1_counter: usize = 0;
    let mut part2_counters: Vec<usize> = Vec::new();
    'line_loop: for line in input.iter() {
        let mut bracket_stack: Vec<Bracket> = Vec::new();
        let mut part2_counter: usize = 0;
        for bracket in line {
            match bracket.end {
                BracketEnd::Opening => bracket_stack.push(*bracket),
                BracketEnd::Closing => match bracket_stack.pop() {
                    None => {
                        part1_counter += match bracket.style {
                            BracketStyle::Round => 3,
                            BracketStyle::Square => 57,
                            BracketStyle::Curly => 1197,
                            BracketStyle::Angled => 25137,
                        };
                        continue 'line_loop;
                    }
                    Some(b) => {
                        let opening_bracket = Bracket {
                            end: BracketEnd::Opening,
                            style: bracket.style,
                        };
                        if b != opening_bracket {
                            part1_counter += match bracket.style {
                                BracketStyle::Round => 3,
                                BracketStyle::Square => 57,
                                BracketStyle::Curly => 1197,
                                BracketStyle::Angled => 25137,
                            };
                            continue 'line_loop;
                        }
                    }
                },
            }
        }
        for open_bracket in bracket_stack.drain(0..).rev() {
            part2_counter *= 5;
            part2_counter += match open_bracket.style {
                BracketStyle::Round => 1,
                BracketStyle::Square => 2,
                BracketStyle::Curly => 3,
                BracketStyle::Angled => 4,
            };
        }
        part2_counters.push(part2_counter);
    }
    println!("Solution for part 1: {}", part1_counter);

    part2_counters.sort_unstable();
    println!(
        "Solution for part 2: {}",
        part2_counters[part2_counters.len() / 2]
    );
}

fn load_input() -> Vec<Vec<Bracket>> {
    let file = File::open("input").expect("No input file found");
    let reader = BufReader::new(file);
    let mut input: Vec<Vec<Bracket>> = Vec::new();
    for line in reader.lines() {
        input.push(
            line.unwrap()
                .split("")
                .filter(|x| !x.is_empty())
                .map(|x| Bracket::from_str(x).unwrap())
                .collect(),
        )
    }
    input
}

#[derive(Debug, Clone, Copy, std::cmp::PartialEq)]
struct Bracket {
    end: BracketEnd,
    style: BracketStyle,
}

impl FromStr for Bracket {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (end, style) = match s {
            "(" => (BracketEnd::Opening, BracketStyle::Round),
            ")" => (BracketEnd::Closing, BracketStyle::Round),
            "[" => (BracketEnd::Opening, BracketStyle::Square),
            "]" => (BracketEnd::Closing, BracketStyle::Square),
            "{" => (BracketEnd::Opening, BracketStyle::Curly),
            "}" => (BracketEnd::Closing, BracketStyle::Curly),
            "<" => (BracketEnd::Opening, BracketStyle::Angled),
            ">" => (BracketEnd::Closing, BracketStyle::Angled),
            _ => return Err(()),
        };
        Ok(Self { end, style })
    }
}

#[derive(Debug, Clone, Copy, std::cmp::PartialEq)]
enum BracketEnd {
    Opening,
    Closing,
}

#[derive(Debug, Clone, Copy, std::cmp::PartialEq)]
enum BracketStyle {
    Round,
    Square,
    Curly,
    Angled,
}
