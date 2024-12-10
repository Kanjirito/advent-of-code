use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let mut input = load_input();
    input.sort_unstable_by(|a, b| b.total.partial_cmp(&a.total).unwrap());
    println!("Highest calories: {}", input[0].total);
    println!(
        "Sum of 3 highest calories: {}",
        input[0..3].iter().map(|e| e.total).sum::<usize>()
    )
}
fn load_input() -> std::vec::Vec<Elf> {
    let file = File::open("input").expect("No input file found");
    let reader = BufReader::new(file);
    let mut elfes: Vec<Elf> = Vec::new();
    let mut cur_foods: Vec<usize> = Vec::new();
    let mut cur_sum: usize = 0;
    for line in reader.lines() {
        match line.expect("Error when reading line").trim() {
            "" => {
                elfes.push(Elf {
                    _food: cur_foods,
                    total: cur_sum,
                });
                cur_foods = Vec::new();
                cur_sum = 0;
            }
            n => {
                let num: usize = n.parse().expect("Not a number");
                cur_foods.push(num);
                cur_sum += num;
            }
        }
    }
    elfes.push(Elf {
        _food: cur_foods,
        total: cur_sum,
    });
    elfes
}

#[derive(Debug)]
struct Elf {
    _food: Vec<usize>,
    total: usize,
}
