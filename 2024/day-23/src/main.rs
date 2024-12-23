use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

type ConnectionsMap = HashMap<String, HashSet<String>>;

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(&input));
    println!("Solution for part 2: {}", part_2(&input));
}

fn part_1(connections: &[(String, String)]) -> usize {
    let map = map_connections(connections);
    let mut counter = HashSet::new();
    for name in map.keys() {
        if name.starts_with('t') {
            counter.extend(find_group_of_3(&map, name));
        }
    }
    counter.len()
}

fn part_2(connections: &[(String, String)]) -> String {
    let map = map_connections(connections);
    let mut result = HashSet::new();
    for name in map.keys() {
        let mut cur = HashSet::new();
        find_whole_group(&map, name, &mut cur);
        if cur.len() > result.len() {
            result = cur;
        }
    }
    let mut vec = Vec::from_iter(result);
    vec.sort_unstable();
    vec.join(",")
}

fn find_group_of_3<'map>(map: &'map ConnectionsMap, start: &'map str) -> Vec<[&'map str; 3]> {
    // Go over every connection of the start computer ("next")
    // Then go over every connection of the "next" computer ("third")
    // If both "start" also connects to "third" then it means they form a group
    let mut solutions = vec![];
    let cur_values = map.get(start).unwrap();
    for conn in cur_values {
        for other in map.get(conn).unwrap() {
            if cur_values.contains(other) {
                let mut v = [start, conn, other];
                v.sort();
                solutions.push(v);
            }
        }
    }
    solutions
}

fn find_whole_group<'map>(map: &'map ConnectionsMap, cur: &'map str, seen: &mut HashSet<String>) {
    seen.insert(cur.to_string());
    for next in map.get(cur).unwrap() {
        if seen.contains(next.as_str()) {
            continue;
        }
        if seen.is_subset(map.get(next).unwrap()) {
            find_whole_group(map, next, seen);
        } else {
            continue;
        }
    }
}

fn map_connections(connections: &[(String, String)]) -> ConnectionsMap {
    let mut map: ConnectionsMap = HashMap::new();

    for conn in connections {
        map.entry(conn.0.clone())
            .or_default()
            .insert(conn.1.clone());
        map.entry(conn.1.clone())
            .or_default()
            .insert(conn.0.clone());
    }

    map
}

fn load_input(name: &str) -> Vec<(String, String)> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let (first, second) = l.split_once('-').unwrap();
            (first.to_string(), second.to_string())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let map = load_input("example");
        assert_eq!(part_1(&map), 7);
    }

    #[test]
    fn part_2_test() {
        let map = load_input("example");
        assert_eq!(part_2(&map), "co,de,ka,ta");
    }
}
