use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
#[cfg(feature = "bench")]
use std::time::Instant;

#[cfg(not(feature = "bench"))]
fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", solve(&input, 40));
    println!("Solution for part 2: {}", solve(&input, 50));
}

#[cfg(feature = "bench")]
fn main() {
    let input = load_input("input");
    let count = 65;

    let mut for_bench = input.clone();
    let mut now = Instant::now();
    for _ in 0..count {
        play_game(&mut for_bench);
    }
    println!("Regular time: {:?}", now.elapsed());

    let mut for_bench = input.clone();
    let mut now = Instant::now();
    for _ in 0..count {
        play_game_no_stack(&mut for_bench);
    }
    println!("No stack time: {:?}", now.elapsed());
}

fn solve(numbers: &[usize], count: usize) -> usize {
    let mut new = numbers.to_vec();
    for _ in 0..count {
        play_game_no_stack(&mut new);
    }
    new.len()
}

#[allow(dead_code)]
fn play_game(numbers: &mut Vec<usize>) {
    let mut new = vec![];
    let mut stack: Vec<usize> = vec![];
    for n in numbers.iter() {
        if let Some(s) = stack.last() {
            if s != n {
                new.push(stack.len());
                new.push(stack[0]);
                stack.clear();
            }
        }
        stack.push(*n)
    }
    new.push(stack.len());
    new.push(stack[0]);
    *numbers = new;
}

fn play_game_no_stack(numbers: &mut Vec<usize>) {
    let mut new = vec![];
    let mut count = 0;
    // Zero is a safe placeholder because it will never be in the number
    let mut prev = 0;
    for n in numbers.iter() {
        if prev == 0 {
            prev = *n;
            count += 1;
        } else if prev != *n {
            new.push(count);
            new.push(prev);
            prev = *n;
            count = 1;
        } else {
            count += 1;
        }
    }
    new.push(count);
    new.push(prev);
    *numbers = new;
}

fn load_input(name: &str) -> Vec<usize> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    BufReader::new(file)
        .lines()
        .flat_map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .map(|d| d.to_digit(10).unwrap() as usize)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_test() {
        let mut input = vec![1];
        play_game(&mut input);
        assert_eq!(input, vec![1, 1]);
        play_game(&mut input);
        assert_eq!(input, vec![2, 1]);
        play_game(&mut input);
        assert_eq!(input, vec![1, 2, 1, 1]);
        play_game(&mut input);
        assert_eq!(input, vec![1, 1, 1, 2, 2, 1]);
        play_game(&mut input);
        assert_eq!(input, vec![3, 1, 2, 2, 1, 1]);
    }

    #[test]
    fn game_no_stack_test() {
        let mut input = vec![1];
        play_game_no_stack(&mut input);
        assert_eq!(input, vec![1, 1]);
        play_game_no_stack(&mut input);
        assert_eq!(input, vec![2, 1]);
        play_game_no_stack(&mut input);
        assert_eq!(input, vec![1, 2, 1, 1]);
        play_game_no_stack(&mut input);
        assert_eq!(input, vec![1, 1, 1, 2, 2, 1]);
        play_game_no_stack(&mut input);
        assert_eq!(input, vec![3, 1, 2, 2, 1, 1]);
    }
}
