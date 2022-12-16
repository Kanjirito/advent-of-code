use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let (valves, _non_zero) = load_input("input");
    let shortest = shortest_paths(&valves);
    // 31 instead of 30 because the first valve will get "opened" while it shouldn't
    println!("{}", traverse(&shortest, 31, 0, "AA", HashSet::new()));
}

fn traverse(
    valves: &HashMap<String, Valve>,
    mut time_left: i64,
    mut cur_pressure: i64,
    cur_valve: &str,
    mut visited: HashSet<String>,
) -> i64 {
    if time_left <= 0 {
        return 0;
    }
    let cur = &valves[cur_valve];
    time_left -= 1;
    cur_pressure += cur.flow * time_left;
    visited.insert(cur_valve.to_string());
    let mut best = cur_pressure;
    for (other, time) in cur.connections.iter() {
        if !visited.contains(other) {
            best = best.max(traverse(
                valves,
                time_left - time,
                cur_pressure,
                other,
                visited.clone(),
            ));
        }
    }
    best
}

/// Gets the shortest paths between valves that are worth going to
///
/// Uses the Floyd–Warshall algorithm to find the shortest paths between valves and then filters
/// out the ones with flow 0. Adds `AA` because that's the starting valve.
fn shortest_paths(valves: &[Valve]) -> HashMap<String, Valve> {
    let id_map: HashMap<String, usize> = valves
        .iter()
        .enumerate()
        .map(|(i, v)| (v.name.clone(), i))
        .collect();
    let v = valves.len();
    let mut dist: Vec<Vec<i64>> = vec![vec![i64::MAX / 2; v]; v];

    for valve in valves {
        let start_id = id_map[&valve.name];
        dist[start_id][start_id] = 0;
        for con in valve.connections.keys() {
            let end_id = id_map[con];
            dist[start_id][end_id] = 1;
        }
    }

    for k in 0..v {
        for i in 0..v {
            for j in 0..v {
                if dist[i][j] > dist[i][k] + dist[k][j] {
                    dist[i][j] = dist[i][k] + dist[k][j];
                }
            }
        }
    }

    // Filters out the unwanted valves
    let mut new_valves: HashMap<String, Valve> = HashMap::new();
    for valve in valves {
        if valve.flow > 0 || valve.name == "AA" {
            let mut connections: HashMap<String, i64> = HashMap::new();
            let id = id_map[&valve.name];
            for (i, distance) in dist[id].iter().enumerate() {
                let other = &valves[i];
                if other.name != valve.name && other.flow > 0 {
                    connections.insert(other.name.to_owned(), *distance);
                }
            }
            let new_valve = Valve {
                name: valve.name.clone(),
                flow: valve.flow,
                connections,
            };
            new_valves.insert(valve.name.clone(), new_valve);
        }
    }
    new_valves
}

fn load_input(name: &str) -> (Vec<Valve>, Vec<String>) {
    let file = File::open(name).expect("No input file found");
    let reader = BufReader::new(file);
    let main_re = Regex::new(r"Valve (\w\w).*=(\d+).*valves? (.*)").unwrap();
    let mut valves = Vec::new();
    let mut non_zero = Vec::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        let captures = main_re.captures(&line).unwrap();
        let name: String = captures.get(1).unwrap().as_str().into();
        let flow: i64 = captures.get(2).unwrap().as_str().parse().unwrap();

        if flow > 0 {
            non_zero.push(name.clone());
        }

        let mut connections: HashMap<String, i64> = HashMap::new();
        for con in captures
            .get(3)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|s| s.to_owned())
        {
            connections.insert(con, 1);
        }
        valves.push(Valve {
            name,
            flow,
            connections,
        });
    }
    (valves, non_zero)
}

#[derive(Debug)]
struct Valve {
    name: String,
    flow: i64,
    connections: HashMap<String, i64>,
}

#[test]
fn example() {
    let (valves, _non_zero) = load_input("example");
    let shortest = shortest_paths(&valves);
    assert_eq!(traverse(&shortest, 31, 0, "AA", HashSet::new()), 1651);
}
