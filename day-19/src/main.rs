use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let blueprints = load_input("input");
    println!("Part 1: {}", part_1(&blueprints));
    println!("Part 2: {}", part_2(&blueprints));
}

fn part_1(blueprints: &[Vec<i64>]) -> i64 {
    let mut result: i64 = 0;
    for (i, blueprint) in blueprints.iter().enumerate() {
        result += ((i as i64) + 1) * solve(blueprint, 24);
    }
    result
}

fn part_2(blueprints: &[Vec<i64>]) -> i64 {
    let mut result: i64 = 1;
    for blueprint in blueprints.iter().take(3) {
        result *= solve(blueprint, 32);
    }
    result
}

fn solve(blueprint: &[i64], time: i64) -> i64 {
    // Blueprint
    //  Ore cost for ore bot
    //  |  Ore cost for clay bot
    //  |  |   Ore and clay cost for obsidian bot
    //  |  |   ||    Ore and obsidian cost for geode bot
    //  |  |   ||    ||
    //  |  |  ┌--┐  ┌--┐
    // [0, 1, 2, 3, 4, 5]
    let ore_bot_cost = blueprint[0];
    let clay_bot_cost = blueprint[1];
    let obsidian_bot_cost = (blueprint[2], blueprint[3]);
    let geode_bot_cost = (blueprint[4], blueprint[5]);
    //                0 = Ore
    //                |  1 = Clay
    //                |  |  2 = Obsidian
    //                |  |  |  3 = Geode
    //                |  |  |  |
    let robots = vec![1, 0, 0, 0];
    let ores = vec![0, 0, 0, 0];

    // Find highest cost for each resource. Used to not make more bots than needed
    let max_ore = *([blueprint[0], blueprint[1], blueprint[2], blueprint[4]]
        .iter()
        .max()
        .unwrap());
    let max_clay = blueprint[3];
    let max_obsidian = blueprint[5];
    let mut max_geodes = 0;
    let mut seen: HashSet<State> = HashSet::new();

    let state = State {
        ores,
        robots,
        time_left: time,
    };
    let mut q = vec![state];
    while let Some(mut cur) = q.pop() {
        seen.insert(cur.clone());
        let cur_ore = cur.ores[0];
        let cur_clay = cur.ores[1];
        let cur_obsidian = cur.ores[2];
        let cur_geode = cur.ores[3];
        let time_left = cur.time_left;
        if time_left == 0 {
            max_geodes = max_geodes.max(cur_geode);
            continue;
        }

        // If there's enough ore and obsidian to make a geode bot make one and don't do anything else
        if cur_ore >= geode_bot_cost.0 && cur_obsidian >= geode_bot_cost.1 {
            // Start production by removing ore
            let new_ores = vec![
                cur_ore - geode_bot_cost.0,
                cur_clay,
                cur_obsidian - geode_bot_cost.1,
                cur_geode,
            ];
            // Create new State
            let mut new_state = State {
                ores: new_ores,
                robots: cur.robots.clone(),
                time_left,
            };
            // Produce the ore from the current bots
            new_state.produce();
            // New geode bot got produced
            new_state.robots[3] += 1;
            if !seen.contains(&new_state) {
                q.push(new_state);
            }
        // If there's enough ore and clay to make an obsidian bot and there's fewer of them
        // than needed to keep up geode bot production, make one
        } else if cur.robots[2] < max_obsidian
            && cur_ore >= obsidian_bot_cost.0
            && cur_clay >= obsidian_bot_cost.1
        {
            let new_ores = vec![
                cur_ore - obsidian_bot_cost.0,
                cur_clay - obsidian_bot_cost.1,
                cur_obsidian,
                cur_geode,
            ];
            let mut new_state = State {
                ores: new_ores,
                robots: cur.robots.clone(),
                time_left,
            };
            new_state.produce();
            new_state.robots[2] += 1;
            if !seen.contains(&new_state) {
                q.push(new_state);
            }
        // Not enough resources for obsidian or geode bot, consider every other possibility
        } else {
            // If not enough ore bots and enough ore try making a new ore bot
            if cur.robots[0] < max_ore && cur_ore >= ore_bot_cost {
                let new_ores = vec![cur_ore - ore_bot_cost, cur_clay, cur_obsidian, cur_geode];
                let mut new_state = State {
                    ores: new_ores,
                    robots: cur.robots.clone(),
                    time_left,
                };
                new_state.produce();
                new_state.robots[0] += 1;
                if !seen.contains(&new_state) {
                    q.push(new_state);
                }
            };
            // If not enough clay bots and enough ore try making a new clay bot
            if cur.robots[1] < max_clay && cur_ore >= clay_bot_cost {
                let new_ores = vec![cur_ore - clay_bot_cost, cur_clay, cur_obsidian, cur_geode];
                let mut new_state = State {
                    ores: new_ores,
                    robots: cur.robots.clone(),
                    time_left,
                };
                new_state.produce();
                new_state.robots[1] += 1;
                if !seen.contains(&new_state) {
                    q.push(new_state);
                }
            };

            // Don't do anything and just wait
            cur.produce();
            if !seen.contains(&cur) {
                q.push(cur);
            }
        }
    }
    max_geodes
}

fn load_input(name: &str) -> Vec<Vec<i64>> {
    let file = File::open(name).expect("No input file found");
    let reader = BufReader::new(file);
    let mut blueprints: Vec<Vec<i64>> = Vec::new();
    let re = Regex::new(r"(\d+) ").unwrap();
    for line in reader.lines().map(|l| l.unwrap()) {
        blueprints.push(
            re.find_iter(&line)
                .map(|f| f.as_str().trim().parse().unwrap())
                .collect(),
        );
    }
    blueprints
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct State {
    ores: Vec<i64>,
    robots: Vec<i64>,
    time_left: i64,
}

impl State {
    fn produce(&mut self) {
        for i in 0..self.robots.len() {
            self.ores[i] += self.robots[i];
        }
        self.time_left -= 1;
    }
}

#[test]
fn example() {
    let input = load_input("example");
    assert_eq!(part_1(&input), 33);
    assert_eq!(part_2(&input), 3348);
}
