use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::BufReader;

use utils::BufReadExt;

fn main() {
    let (wires, z_wires, mut gates) = load_input("input");
    println!("Solution for part 1: {}", part_1(&wires, &z_wires));
    println!("Solution for part 2: {}", part_2(&mut gates));
}

fn part_1(wires: &HashMap<String, Value>, z_wires: &[String]) -> u64 {
    let mut cur_wires = wires.clone();
    let mut visited = HashSet::new();
    for z in z_wires {
        solve(z, &mut cur_wires, &mut visited);
    }
    get_result(&cur_wires, 'z')
}

fn get_result(wires: &HashMap<String, Value>, prefix: char) -> u64 {
    let mut solution = vec![];
    for i in 0.. {
        match wires.get(&format!("{}{i:02}", prefix)) {
            Some(v) => solution.push(v.as_bool()),
            None => break,
        }
    }
    solution.reverse();
    u64::from_str_radix(
        &solution
            .into_iter()
            .map(|n| match n {
                true => '1',
                false => '0',
            })
            .collect::<String>(),
        2,
    )
    .unwrap()
}

fn part_2(gates: &mut HashSet<Gate>) -> String {
    // println!("{:#?}", gates);
    let mut solution = Vec::new();
    solution.extend(swap_outputs(
        Gate::new_empty(GateType::Xor, ["x11", "y11"]),
        Gate::new_empty(GateType::And, ["x11", "y11"]),
        gates,
    ));
    solution.extend(swap_outputs(
        Gate::new_empty(GateType::Xor, ["rqq", "pvh"]),
        Gate::new_empty(GateType::Or, ["pqv", "ffg"]),
        gates,
    ));

    solution.extend(swap_outputs(
        Gate::new_empty(GateType::Xor, ["kfq", "qgs"]),
        Gate::new_empty(GateType::And, ["kfq", "qgs"]),
        gates,
    ));

    solution.extend(swap_outputs(
        Gate::new_empty(GateType::And, ["x39", "y39"]),
        Gate::new_empty(GateType::Xor, ["sbq", "hkg"]),
        gates,
    ));
    let mut c = String::from("wbd");
    for i in 1..=44 {
        let a = format!("x{:02}", i);
        let b = format!("y{:02}", i);
        c = full_adder(a, b, c, gates)
    }
    solution.sort();
    solution.join(",")
}

fn swap_outputs(first: Gate, second: Gate, gates: &mut HashSet<Gate>) -> [String; 2] {
    let mut one = gates.take(&first).unwrap();
    let mut two = gates.take(&second).unwrap();
    let tmp_one = one.out;
    let tmp_two = two.out;
    one.out = tmp_two.clone();
    two.out = tmp_one.clone();
    gates.insert(one);
    gates.insert(two);
    [tmp_one, tmp_two]
}

fn full_adder(a: String, b: String, c: String, gates: &HashSet<Gate>) -> String {
    // println!("A: {a}, B: {b} C: {c}");
    let mut a_b_sort = [a.clone(), b];
    a_b_sort.sort();
    let x = match gates.get(&Gate {
        kind: GateType::Xor,
        input: a_b_sort.clone(),
        out: String::new(),
    }) {
        Some(g) => &g.out,
        None => panic!("No A XOR B = X"),
    };

    // println!("X: {x}");

    let y = match gates.get(&Gate {
        kind: GateType::And,
        input: a_b_sort,
        out: String::new(),
    }) {
        Some(g) => &g.out,
        None => panic!("No A AND B = Y"),
    };

    // println!("Y: {y}");

    let mut x_c_sort = [x.to_string(), c.to_string()];
    x_c_sort.sort();

    let w = match gates.get(&Gate {
        kind: GateType::And,
        input: x_c_sort.clone(),
        out: String::new(),
    }) {
        Some(g) => &g.out,
        None => panic!("No X AND C = W"),
    };

    // println!("W: {w}");

    match gates.get(&Gate {
        kind: GateType::Xor,
        input: x_c_sort,
        out: String::new(),
    }) {
        Some(g) => {
            if g.out != format!("z{}", &a[1..]) {
                panic!("WRONG Z: {}", g.out)
            }
        }
        None => panic!("No X XOR C = Z"),
    };

    let mut w_y_sort = [w.to_string(), y.to_string()];
    w_y_sort.sort();

    match gates.get(&Gate {
        kind: GateType::Or,
        input: w_y_sort,
        out: String::new(),
    }) {
        Some(g) => g.out.clone(),
        None => panic!("No W OR Y = X"),
    }
}


fn solve(value: &str, wires: &mut HashMap<String, Value>, visited: &mut HashSet<Gate>) -> bool {
    match wires.get_mut(value).unwrap().clone() {
        Value::Gate(gate) => {
            let first = solve(&gate.input[0], wires, visited);
            let second = solve(&gate.input[1], wires, visited);
            let r = match gate.kind {
                GateType::And => first && second,
                GateType::Or => first | second,
                GateType::Xor => first != second,
            };
            visited.insert(gate);
            wires.insert(value.to_string(), Value::Value(r));
            r
        }
        Value::Value(v) => v,
    }
}

fn load_input(name: &str) -> (HashMap<String, Value>, Vec<String>, HashSet<Gate>) {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut lines = reader.lines_unwrap();

    let mut wires = HashMap::new();
    let mut z_wires = vec![];

    for l in &mut lines {
        if l.is_empty() {
            break;
        }
        let (name, value) = l.split_once(": ").unwrap();
        if name.starts_with('z') {
            z_wires.push(name.to_string());
        }
        wires.insert(
            name.to_string(),
            Value::Value(match value {
                "0" => false,
                "1" => true,
                _ => unreachable!(),
            }),
        );
    }

    let mut gates = HashSet::new();

    for l in lines {
        let gate = Gate::from(l);
        if gate.out.starts_with('z') {
            z_wires.push(gate.out.clone());
        }
        gates.insert(gate.clone());
        wires.insert(gate.out.to_string(), Value::Gate(gate));
    }
    z_wires.sort_unstable();
    (wires, z_wires, gates)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    Gate(Gate),
    Value(bool),
}

impl Value {
    fn as_bool(&self) -> bool {
        match self {
            Value::Gate(_) => unreachable!(),
            Value::Value(v) => *v,
        }
    }
}

impl From<char> for Value {
    fn from(value: char) -> Self {
        Self::Value(match value {
            '0' => false,
            '1' => true,
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Clone)]
struct Gate {
    kind: GateType,
    input: [String; 2],
    out: String,
}

impl Gate {
    fn new_empty(kind: GateType, input: [&str; 2]) -> Self {
        let mut input = [input[0].to_string(), input[1].to_string()];
        input.sort();
        Self {
            kind,
            input,
            out: String::new(),
        }
    }
}

impl PartialEq for Gate {
    fn eq(&self, other: &Gate) -> bool {
        self.kind == other.kind && self.input == other.input
    }
}

impl Eq for Gate {}

impl Hash for Gate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
        self.input.hash(state);
    }
}

impl From<String> for Gate {
    fn from(value: String) -> Self {
        let mut split = value.split(' ');
        let in_1 = split.next().unwrap().to_string();
        let kind = GateType::from(split.next().unwrap());
        let in_2 = split.next().unwrap().to_string();
        split.next();
        let out = split.next().unwrap().to_string();
        let mut input = [in_1, in_2];
        input.sort();
        Self { kind, input, out }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GateType {
    And,
    Or,
    Xor,
}

impl From<&str> for GateType {
    fn from(value: &str) -> Self {
        match value {
            "AND" => Self::And,
            "OR" => Self::Or,
            "XOR" => Self::Xor,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test_1() {
        let (wires, z_wires, _) = load_input("example");
        assert_eq!(part_1(&wires, &z_wires), 4);
    }

    #[test]
    fn part_1_test_2() {
        let (wires, z_wires, _) = load_input("example2");
        assert_eq!(part_1(&wires, &z_wires), 2024);
    }
}
