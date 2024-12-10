use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use regex::Regex;

#[derive(Debug)]
struct Ingredient {
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize,
}

impl From<&str> for Ingredient {
    fn from(value: &str) -> Self {
        let (_, rest) = value.split_once(':').unwrap();
        let reg = Regex::new(r"-?\d+").unwrap();
        let mut result = reg
            .find_iter(rest)
            .map(|r| r.as_str().parse::<isize>().unwrap());
        Self {
            capacity: result.next().unwrap(),
            durability: result.next().unwrap(),
            flavor: result.next().unwrap(),
            texture: result.next().unwrap(),
            calories: result.next().unwrap(),
        }
    }
}

impl Ingredient {
    fn scores(&self, count: isize) -> (isize, isize, isize, isize, isize) {
        (
            self.capacity * count,
            self.durability * count,
            self.flavor * count,
            self.texture * count,
            self.calories * count,
        )
    }
}

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", solve(&input, false));
    println!("Solution for part 2: {}", solve(&input, true));
}

fn solve(ingredients: &[Ingredient], part_2: bool) -> isize {
    let mut q: Vec<Vec<isize>> = vec![];
    let mut highest = isize::MIN;

    for i in 1..=100 {
        q.push(vec![i]);
    }

    while let Some(cur) = q.pop() {
        if cur.len() == ingredients.len() && cur.iter().sum::<isize>() == 100 {
            let mut cur_total = [0, 0, 0, 0, 0];
            for (i, ingredient) in ingredients.iter().enumerate() {
                let score = ingredient.scores(cur[i]);
                cur_total[0] += score.0;
                cur_total[1] += score.1;
                cur_total[2] += score.2;
                cur_total[3] += score.3;
                cur_total[4] += score.4;
            }

            let total = cur_total
                .iter()
                .take(4)
                .filter(|x| x.is_positive())
                .product();
            if part_2 {
                if cur_total[4] == 500 {
                    highest = highest.max(total);
                }
            } else {
                highest = highest.max(total);
            }
        }

        for _ in &ingredients[cur.len()..] {
            let left = 100 - cur.iter().sum::<isize>();
            for i in 1..=left {
                let mut new = cur.clone();
                new.push(i);
                q.push(new);
            }
        }
    }
    highest
}

fn load_input(name: &str) -> Vec<Ingredient> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.unwrap().as_str().into()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> Vec<Ingredient> {
        vec![
            "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8".into(),
            "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3".into(),
        ]
    }

    #[test]
    fn score() {
        let input = example_input();
        assert_eq!(input[0].scores(10), (-10, -20, 60, 30, 80));
        assert_eq!(input[1].scores(10), (20, 30, -20, -10, 30))
    }

    #[test]
    fn part_1_test() {
        let input = example_input();
        assert_eq!(solve(&input, false), 62842880);
    }

    #[test]
    fn part_2_test() {
        let input = example_input();
        assert_eq!(solve(&input, true), 57600000);
    }
}
