use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
// 97 - 122

fn main() {
    let input = load_input("input");
    let first = solve(&input);
    println!("Solution for part 1: {}", first);
    println!("Solution for part 2: {}", solve(&str_to_u8(&first)));
}

fn get_passw(pass: &[u8]) -> String {
    let fixed: Vec<u8> = pass.iter().map(|n| n + 97).collect();
    String::from_utf8_lossy(&fixed).to_string()
}

fn check_passw(pass: &[u8]) -> bool {
    // 99 is a safe placeholder because chars can only be up to 25
    let mut stack = [99, 99];
    let mut three_in_row = false;
    let mut pair_cout = 0;
    for n in pass {
        // i / o / l
        if *n == 8 || *n == 11 || *n == 14 {
            return false;
        }
        if !three_in_row
            && n.saturating_sub(stack[1]) == 1
            && stack[1].saturating_sub(stack[0]) == 1
        {
            three_in_row = true;
        }

        if *n == stack[1] && *n != stack[0] {
            pair_cout += 1;
        }
        stack.rotate_left(1);
        stack[1] = *n;
    }
    three_in_row && pair_cout >= 2
}

fn solve(pass: &[u8]) -> String {
    let mut cur = pass.to_vec();
    // The next password can't be this password so we increment right away
    increment_passw(&mut cur);
    while !check_passw(&cur) {
        increment_passw(&mut cur);
    }
    get_passw(&cur)
}

/// Increments the password, returns true if the first char went over
fn increment_passw(pass: &mut [u8]) -> bool {
    let mut ran_out = true;

    for i in (0..pass.len()).rev() {
        let n = pass[i];

        // If next letter is i/o/l...
        if n == 7 || n == 10 || n == 13 {
            // ...skip it...
            pass[i] += 2;
            // ...and set all of the trailing chars to 0
            for z in pass[i + 1..].iter_mut() {
                *z = 0;
            }
        } else {
            pass[i] += 1;
        }

        let n_ref = pass.get_mut(i).unwrap();
        let div = *n_ref / 26;
        *n_ref %= 26;
        if div == 0 {
            ran_out = false;
            break;
        }
    }
    ran_out
}

fn load_input(name: &str) -> Vec<u8> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    str_to_u8(&reader.lines().next().unwrap().unwrap())
}

fn str_to_u8(s: &str) -> Vec<u8> {
    s.as_bytes().iter().map(|n| n - 97).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment() {
        let mut input = vec![0];
        let result = (0..23)
            .map(|_| {
                increment_passw(&mut input);
                input[0]
            })
            .fold(Vec::new(), |mut v, r| {
                v.push(r);
                v
            });
        assert_eq!(
            result,
            vec![1, 2, 3, 4, 5, 6, 7, 9, 10, 12, 13, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 0]
        );
    }

    #[test]
    fn test_overlow_increment() {
        let mut input = vec![0, 0, 25, 24];
        increment_passw(&mut input);
        assert_eq!(input, vec![0, 0, 25, 25]);
        increment_passw(&mut input);
        assert_eq!(input, vec![0, 1, 0, 0]);
    }

    #[test]
    fn test_end_of_passw() {
        let mut input = vec![25, 24];
        assert!(!increment_passw(&mut input));
        assert!(increment_passw(&mut input));
    }
    #[test]
    fn failed_examples() {
        assert!(!check_passw(&str_to_u8("hijklmmn")));
        assert!(!check_passw(&str_to_u8("abbceffg")));
        assert!(!check_passw(&str_to_u8("abbcegjk")));
    }

    #[test]
    fn print_test() {
        assert_eq!(get_passw(&[0]), "a");
        assert_eq!(get_passw(&[25]), "z");
    }

    #[test]
    fn part_1_test() {
        assert_eq!(solve(&str_to_u8("abcdefgh")), "abcdffaa");
        assert_eq!(solve(&str_to_u8("ghijklmn")), "ghjaabcc");
    }
}
