use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", get_len(&input, false));
    println!("Solution for part 2: {}", get_len(&input, true));
}

fn load_input(name: &str) -> Vec<char> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    let _ = reader.read_line(&mut buf);
    buf.trim().chars().collect()
}

fn get_len(to_check: &[char], recursive: bool) -> usize {
    let mut cur_i = 0;
    let mut cur_len = 0;
    while cur_i < to_check.len() {
        match to_check[cur_i] {
            '(' => {
                cur_i += 1;
                let mut marker = vec![];
                while to_check[cur_i] != ')' {
                    marker.push(to_check[cur_i]);
                    cur_i += 1;
                }
                cur_i += 1;
                let tmp: String = marker.into_iter().collect();
                let (x, y) = tmp.split_once('x').unwrap();
                let l: usize = x.parse().unwrap();
                let t: usize = y.parse().unwrap();
                if recursive {
                    cur_len += t * get_len(&to_check[cur_i..cur_i + l], recursive)
                } else {
                    cur_len += l * t;
                }
                cur_i += l;
            }
            _ => {
                cur_i += 1;
                cur_len += 1;
            }
        }
    }
    cur_len
}
