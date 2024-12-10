use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let (mut stacks, moves) = load_input();
    println!("Part 1: {}", part_1(&mut stacks.clone(), &moves));
    println!("Part 2: {}", part_2(&mut stacks, &moves));
}

fn part_1(stacks: &mut [Vec<char>], moves: &[Move]) -> String {
    for m in moves {
        for _ in 0..m.count {
            let tmp = stacks[m.from].pop().expect("Popped from empty stack");
            stacks[m.to].push(tmp);
        }
    }
    let mut result: Vec<char> = Vec::new();
    for stack in stacks {
        result.push(*stack.iter().last().unwrap())
    }
    result.iter().collect()
}

fn part_2(stacks: &mut [Vec<char>], moves: &[Move]) -> String {
    for m in moves {
        let mut tmp_stack: Vec<char> = Vec::new();
        for _ in 0..m.count {
            tmp_stack.push(stacks[m.from].pop().expect("Popped from empty stack"));
        }

        while let Some(value) = tmp_stack.pop() {
            stacks[m.to].push(value);
        }
    }
    let mut result: Vec<char> = Vec::new();
    for stack in stacks {
        result.push(*stack.iter().last().unwrap())
    }
    result.iter().collect()
}

fn load_input() -> (Vec<Vec<char>>, Vec<Move>) {
    let file = File::open("parsed").expect("No input file found");
    let mut reader = BufReader::new(file).lines();
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..9 {
        stacks.push(reader.next().unwrap().unwrap().chars().collect())
    }
    reader.next();

    let mut moves: Vec<Move> = Vec::new();
    for line in reader.map(|l| l.unwrap()) {
        let split: Vec<usize> = line.split(' ').map(|num| num.parse().unwrap()).collect();
        moves.push(Move {
            count: split[0],
            from: split[1] - 1,
            to: split[2] - 1,
        })
    }
    (stacks, moves)
}

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}
