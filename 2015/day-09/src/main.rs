use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct Connection {
    target: String,
    distance: usize,
}

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", solve(&input, false));
    println!("Solution for part 2: {}", solve(&input, true));
}

fn solve(conns: &HashMap<String, Vec<Connection>>, max: bool) -> usize {
    // Just goes over every possibility since the input is short
    let mut q: Vec<(Vec<&str>, usize)> = vec![];
    let mut record = if max { 0 } else { usize::MAX };
    for start in conns.keys() {
        q.push((vec![start], 0));
    }
    while let Some((visited, cur_distance)) = q.pop() {
        let cur = visited.last().unwrap().to_owned();
        let mut moved = false;

        for conn in conns.get(cur).unwrap() {
            if !visited.contains(&conn.target.as_str()) {
                let new_distance = cur_distance + conn.distance;
                let mut new_visited = visited.clone();
                new_visited.push(&conn.target);
                q.push((new_visited, new_distance));
                moved = true;
            }
        }
        if !moved {
            // No more valid targets so it's the end
            if max {
                record = record.max(cur_distance);
            } else {
                record = record.min(cur_distance);
            }
        }
    }
    record
}

fn load_input(name: &str) -> HashMap<String, Vec<Connection>> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut connections = HashMap::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        let mut split = line.split(' ');
        let cur = split.next().unwrap().to_owned();
        split.next();
        let target = split.next().unwrap().to_owned();
        split.next();
        let distance = split.next().unwrap().parse().unwrap();
        (*connections.entry(cur.clone()).or_insert(Vec::new())).push(Connection {
            target: target.clone(),
            distance,
        });
        (*connections.entry(target).or_insert(Vec::new())).push(Connection {
            target: cur,
            distance,
        });
    }
    connections
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = load_input("example");
        assert_eq!(solve(&input, false), 605);
    }

    #[test]
    fn part_2_test() {
        let input = load_input("example");
        assert_eq!(solve(&input, true), 982);
    }
}
