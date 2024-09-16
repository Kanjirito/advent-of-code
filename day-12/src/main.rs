use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use jzon::JsonValue;

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", solve(&input, false).unwrap());
    println!("Solution for part 2: {}", solve(&input, true).unwrap());
}

fn solve(json: &JsonValue, part_2: bool) -> Option<i64> {
    let mut counter = 0;
    match json {
        JsonValue::Null | JsonValue::String(_) | JsonValue::Boolean(_) => (),
        JsonValue::Short(s) => {
            if part_2 && s == "red" {
                return None;
            }
        }
        JsonValue::Number(n) => counter += n.as_fixed_point_i64(0).unwrap(),
        JsonValue::Array(ar) => {
            for e in ar {
                counter += solve(e, part_2).unwrap_or(0);
            }
        }
        JsonValue::Object(ob) => {
            for (_, v) in ob.iter() {
                if let Some(n) = solve(v, part_2) {
                    counter += n;
                } else {
                    return Some(0);
                }
            }
        }
    }
    Some(counter)
}

fn load_input(name: &str) -> JsonValue {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    jzon::parse(&(reader.lines().next().unwrap().unwrap())).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(solve(&jzon::array![1, 2, 3], false).unwrap(), 6);
        assert_eq!(solve(&jzon::array![[[3]]], false).unwrap(), 3);
        assert_eq!(solve(&jzon::array![], false).unwrap(), 0);
        assert_eq!(solve(&jzon::object! {"a":[-1,1]}, false).unwrap(), 0);
        assert_eq!(solve(&jzon::object! {}, false).unwrap(), 0);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(solve(&dbg!(jzon::array![1, 2, 3]), true).unwrap(), 6);
        assert_eq!(
            solve(&dbg!(jzon::array! [1,{"c":"red","b":2},3]), true).unwrap(),
            4
        );
        assert_eq!(
            solve(&dbg!(jzon::object! {"d":"red","e":[1,2,3,4],"f":5}), true).unwrap(),
            0
        );
        assert_eq!(solve(&dbg!(jzon::array![1, "red", 5]), true).unwrap(), 6);
    }
}
