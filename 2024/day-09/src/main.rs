use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::mem::replace;

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&input));
}

fn part_1(input: &[(Value, usize)]) -> usize {
    let list = expand_list(input);
    let mut counter = 0;

    let mut x = 0;
    let mut y = list.len() - 1;
    let mut cur_i = 0;
    while x <= y {
        if let Value::File(n) = list[x] {
            counter += cur_i * n;
            x += 1;
        } else if let Value::File(n) = list[y] {
            counter += cur_i * n;
            x += 1;
            y -= 1;
        } else {
            y -= 1;
            continue;
        }
        cur_i += 1;
    }
    counter
}

fn part_2(input: &[(Value, usize)]) -> usize {
    let max_id = input
        .iter()
        .rev()
        .find_map(|(v, _)| {
            if let Value::File(n) = v {
                Some(n)
            } else {
                None
            }
        })
        .unwrap();
    let mut cur_id = *max_id;
    let mut line = input.to_vec();

    'outer: while cur_id > 0 {
        for x in (0..line.len()).rev() {
            let (value, len) = line[x];
            match value {
                Value::File(n) if n == cur_id => {
                    for y in 0..x {
                        match line[y] {
                            (Value::Empty, l) if l >= len => {
                                let _ = replace(&mut line[x], (Value::Empty, len));
                                let _ = replace(&mut line[y], (value, len));
                                if l - len > 0 {
                                    line.insert(y + 1, (Value::Empty, l - len));
                                }
                                cur_id -= 1;
                                continue 'outer;
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        cur_id -= 1;
    }

    let mut counter = 0;
    let mut cur_pos = 0;
    for (v, l) in line {
        for _ in 0..l {
            match v {
                Value::File(n) => {
                    counter += n * cur_pos;
                }
                Value::Empty => {}
            }
            cur_pos += 1;
        }
    }
    counter
}

fn load_input(name: &str) -> Vec<(Value, usize)> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut numbers = vec![];
    for line in reader.lines().map(|l| l.unwrap()) {
        for (i, c) in line.chars().enumerate() {
            let n = c.to_digit(10).unwrap() as usize;
            if n == 0 {
                continue;
            } else if i % 2 == 0 {
                numbers.push((Value::File(i / 2), n));
            } else {
                numbers.push((Value::Empty, n));
            }
        }
    }
    numbers
}

fn expand_list(input: &[(Value, usize)]) -> Vec<Value> {
    let mut list = vec![];
    for (value, count) in input {
        for _ in 0..*count {
            list.push(*value);
        }
    }
    list
}

#[derive(Debug, Clone, Copy)]
enum Value {
    Empty,
    File(usize),
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = load_input("example");
        assert_eq!(part_1(&input), 1928);
    }

    #[test]
    fn part_2_test() {
        let input = load_input("example");
        assert_eq!(part_2(&input), 2858);
    }
}
