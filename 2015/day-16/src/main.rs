#![allow(unused_variables, dead_code)]
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REG: Regex = Regex::new(r#"(\w+): (\d+)"#).unwrap();
    static ref BEST_AUNT: Aunt = Aunt {
        number: 999,
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };
}

#[derive(Debug)]
struct Aunt {
    number: usize,
    children: Option<usize>,
    cats: Option<usize>,
    samoyeds: Option<usize>,
    pomeranians: Option<usize>,
    akitas: Option<usize>,
    vizslas: Option<usize>,
    goldfish: Option<usize>,
    trees: Option<usize>,
    cars: Option<usize>,
    perfumes: Option<usize>,
}

impl Aunt {
    fn compare(&self, other: &Aunt) -> bool {
        if let Some(x) = self.children {
            if x != other.children.unwrap() {
                return false;
            }
        }
        if let Some(x) = self.cats {
            if x != other.cats.unwrap() {
                return false;
            }
        }
        if let Some(x) = self.samoyeds {
            if x != other.samoyeds.unwrap() {
                return false;
            }
        }
        if let Some(x) = self.pomeranians {
            if x != other.pomeranians.unwrap() {
                return false;
            }
        }
        if let Some(x) = self.akitas {
            if x != other.akitas.unwrap() {
                return false;
            }
        }
        if let Some(x) = self.vizslas {
            if x != other.vizslas.unwrap() {
                return false;
            }
        }
        if let Some(x) = self.goldfish {
            if x != other.goldfish.unwrap() {
                return false;
            }
        }
        if let Some(x) = self.trees {
            if x != other.trees.unwrap() {
                return false;
            }
        }
        if let Some(x) = self.cars {
            if x != other.cars.unwrap() {
                return false;
            }
        }
        if let Some(x) = self.perfumes {
            if x != other.perfumes.unwrap() {
                return false;
            }
        }

        true
    }

    fn compare_2(&self, other: &Aunt) -> bool {
        if let Some(x) = self.children {
            if x != other.children.unwrap() {
                return false;
            }
        }
        if let Some(x) = self.cats {
            if x <= other.cats.unwrap() {
                return false;
            }
        }
        if let Some(x) = self.samoyeds {
            if x != other.samoyeds.unwrap() {
                return false;
            }
        }
        if let Some(x) = self.pomeranians {
            if x >= other.pomeranians.unwrap() {
                return false;
            }
        }
        if let Some(x) = self.akitas {
            if x != other.akitas.unwrap() {
                return false;
            }
        }
        if let Some(x) = self.vizslas {
            if x != other.vizslas.unwrap() {
                return false;
            }
        }
        if let Some(x) = self.goldfish {
            if x >= other.goldfish.unwrap() {
                return false;
            }
        }
        if let Some(x) = self.trees {
            if x <= other.trees.unwrap() {
                return false;
            }
        }
        if let Some(x) = self.cars {
            if x != other.cars.unwrap() {
                return false;
            }
        }
        if let Some(x) = self.perfumes {
            if x != other.perfumes.unwrap() {
                return false;
            }
        }

        true
    }
}

impl From<&str> for Aunt {
    fn from(value: &str) -> Self {
        let (number, rest) = value.split_once(": ").unwrap();
        let result = REG.captures_iter(rest);
        let mut children = None;
        let mut cats = None;
        let mut samoyeds = None;
        let mut pomeranians = None;
        let mut akitas = None;
        let mut vizslas = None;
        let mut goldfish = None;
        let mut trees = None;
        let mut cars = None;
        let mut perfumes = None;

        for matc in result {
            *match matc.get(1).unwrap().as_str() {
                "children" => &mut children,
                "cats" => &mut cats,
                "samoyeds" => &mut samoyeds,
                "pomeranians" => &mut pomeranians,
                "akitas" => &mut akitas,
                "vizslas" => &mut vizslas,
                "goldfish" => &mut goldfish,
                "trees" => &mut trees,
                "cars" => &mut cars,
                "perfumes" => &mut perfumes,
                &_ => unreachable!(),
            } = matc.get(2).map(|x| x.as_str().parse::<usize>().unwrap());
        }
        Self {
            number: number
                .strip_prefix("Sue ")
                .unwrap()
                .parse::<usize>()
                .unwrap(),
            children,
            cats,
            samoyeds,
            pomeranians,
            akitas,
            vizslas,
            goldfish,
            trees,
            cars,
            perfumes,
        }
    }
}

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&input));
}

fn part_1(aunts: &[Aunt]) -> usize {
    for aunt in aunts {
        if aunt.compare(&BEST_AUNT) {
            return aunt.number;
        }
    }
    unreachable!()
}

fn part_2(aunts: &[Aunt]) -> usize {
    for aunt in aunts {
        if aunt.compare_2(&BEST_AUNT) {
            return aunt.number;
        }
    }
    unreachable!()
}

fn load_input(name: &str) -> Vec<Aunt> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.unwrap().as_str().into()).collect()
}
