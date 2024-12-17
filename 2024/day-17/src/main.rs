use std::fs::File;
use std::io::BufReader;

use utils::BufReadExt;

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", part_1(input));
    // println!("Solution for part 2: {}", part_2(&input));
}

fn part_1(mut computer: Computer) -> String {
    computer.run_program();
    // dbg!(&computer);
    computer.get_result()
}

fn part_2() {
    todo!()
}

fn load_input(name: &str) -> Computer {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    let mut lines = reader.lines_unwrap();
    Computer {
        reg_a: lines
            .next()
            .unwrap()
            .strip_prefix("Register A: ")
            .unwrap()
            .parse()
            .unwrap(),
        reg_b: lines
            .next()
            .unwrap()
            .strip_prefix("Register B: ")
            .unwrap()
            .parse()
            .unwrap(),
        reg_c: lines
            .next()
            .unwrap()
            .strip_prefix("Register C: ")
            .unwrap()
            .parse()
            .unwrap(),
        program: lines
            .nth(1)
            .unwrap()
            .strip_prefix("Program: ")
            .unwrap()
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect(),
        cur_op: 0,
        result: vec![],
    }
}

#[derive(Debug, Clone)]
struct Computer {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    program: Vec<usize>,
    cur_op: usize,
    result: Vec<String>,
}

impl Computer {
    fn new(a: usize, b: usize, c: usize, program: Vec<usize>) -> Self {
        Self {
            reg_a: a,
            reg_b: b,
            reg_c: c,
            program,
            cur_op: 0,
            result: vec![],
        }
    }

    fn run_program(&mut self) {
        while self.do_instruction() {}
    }

    fn do_instruction(&mut self) -> bool {
        dbg!(&self);
        let op_literal = self.program[self.cur_op + 1];
        let op_combo = self.get_combo_value();
        let cur_literal = self.program[self.cur_op];
        match Op::from(cur_literal) {
            Op::Adv => {
                self.reg_a /= 2_usize.pow(op_combo as u32);
            }
            Op::Bxl => {
                self.reg_b ^= op_literal;
            }
            Op::Bst => {
                self.reg_b = op_combo % 8;
            }
            Op::Jnz => {
                println!("JUMP");
                if self.reg_a != 0 {
                    self.cur_op = op_literal;
                    return self.cur_op < self.program.len();
                }
            }
            Op::Bxc => {
                self.reg_b ^= self.reg_c;
            }
            Op::Out => {
                self.result.push((op_combo % 8).to_string());
            }
            Op::Bdv => {
                self.reg_b = self.reg_a / 2_usize.pow(op_combo as u32);
            }
            Op::Cdv => {
                self.reg_c = self.reg_a / 2_usize.pow(op_combo as u32);
            }
        }
        self.cur_op += 2;
        self.cur_op < self.program.len()
    }

    fn get_combo_value(&self) -> usize {
        match self.program[self.cur_op + 1] {
            v @ 0..=3 => v,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            7 => usize::MAX,
            _ => unreachable!(),
        }
    }

    fn get_result(&self) -> String {
        self.result.join(",")
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Op {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

// impl From<Op> for usize {
//     fn from(value: Op) -> Self {
//         match value {
//             Op::Adv => 0,
//             Op::Bxl => 1,
//             Op::Bst => 2,
//             Op::Jnz => 3,
//             Op::Bxc => 4,
//             Op::Out => 5,
//             Op::Bdv => 6,
//             Op::Cdv => 7,
//         }
//     }
// }

impl From<usize> for Op {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test_1() {
        let mut computer = Computer::new(0, 0, 9, vec![2, 6]);
        computer.run_program();
        assert_eq!(computer.get_result(), "");
        assert_eq!(computer.reg_b, 1);
    }

    #[test]
    fn part_1_test_2() {
        let computer = Computer::new(10, 0, 0, vec![5, 0, 5, 1, 5, 4]);
        assert_eq!(&part_1(computer), "0,1,2");
    }

    #[test]
    fn part_1_test_3() {
        let mut computer = Computer::new(2024, 0, 0, vec![0, 1, 5, 4, 3, 0]);
        computer.run_program();
        assert_eq!(computer.get_result(), "4,2,5,6,7,7,7,7,3,1,0");
        assert_eq!(computer.reg_a, 0);
    }

    #[test]
    fn part_1_test_4() {
        let mut computer = Computer::new(0, 29, 0, vec![1, 7]);
        computer.run_program();
        dbg!(&computer);
        assert_eq!(computer.reg_b, 26);
    }

    #[test]
    fn part_1_test_5() {
        let mut computer = Computer::new(0, 2024, 43690, vec![4, 0]);
        computer.run_program();
        assert_eq!(computer.reg_b, 44354);
    }

    #[test]
    fn part_1_test_6() {
        let computer = load_input("example");
        assert_eq!(&part_1(computer), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part_2_test() {
        // todo!()
    }
}
