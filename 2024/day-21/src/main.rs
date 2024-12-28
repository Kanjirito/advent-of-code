use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let input = load_input("input");
    println!("Solution for part 1: {}", solve(&input, 2));
    println!("Solution for part 2: {}", solve(&input, 25));
}

fn solve(codes: &[Vec<Keypad>], reps: usize) -> usize {
    let mut counter = 0;

    let mut cache = HashMap::new();
    for code in codes {
        let cur = keypad_to_robot(code);
        counter += robot_to_robot_recur(&cur, reps + 1, &mut cache) * code_to_number(code);
    }
    counter
}

fn keypad_to_robot(code: &[Keypad]) -> Vec<RobotKeypad> {
    let mut cur_position = Keypad::A;
    let mut path = vec![];
    for &button in code {
        path.extend(cur_position.optimal_path(&button));
        path.push(RobotKeypad::A);
        cur_position = button;
    }
    path
}

fn robot_to_robot_recur(
    cur_code: &[RobotKeypad],
    left: usize,
    cache: &mut HashMap<(RobotKeypad, RobotKeypad, usize), usize>,
) -> usize {
    if left == 0 {
        return 1;
    }
    let mut cur_size = 0;
    let mut prev = RobotKeypad::A;
    for &button in cur_code {
        cur_size += if cache.contains_key(&(button, prev, left)) {
            *cache.get(&(button, prev, left)).unwrap()
        } else {
            let r = robot_to_robot_recur(&prev.optimal_path(&button), left - 1, cache);
            cache.insert((button, prev, left), r);
            r
        };
        prev = button;
    }

    cur_size
}

fn code_to_number(code: &[Keypad]) -> usize {
    code[..3]
        .iter()
        .map(|x| x.to_string())
        .collect::<String>()
        .parse()
        .unwrap()
}

#[allow(dead_code)]
fn print_vec<T>(arg: &[T])
where
    T: Display,
{
    for i in arg {
        print!("{}", i);
    }
    println!();
}

fn load_input(name: &str) -> Vec<Vec<Keypad>> {
    let file = File::open(name).unwrap_or_else(|_| panic!("No \"{}\" file found", name));
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.unwrap().chars().map(Keypad::from).collect())
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Keypad {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
}

impl Keypad {
    // ^ >
    // V
    // <
    fn optimal_path(&self, other: &Self) -> Vec<RobotKeypad> {
        use RobotKeypad::*;
        let paths: [[Vec<RobotKeypad>; 11]; 11] = [
            // 0
            [
                vec![],
                vec![Up, Left],
                vec![Up],
                vec![Up, Right],
                vec![Up, Up, Left],
                vec![Up, Up],
                vec![Up, Right],
                vec![Up, Up, Up, Left],
                vec![Up, Up, Up],
                vec![Up, Up, Up, Right],
                vec![Right],
            ],
            // 1
            [
                vec![Right, Down],
                vec![],
                vec![Right],
                vec![Right, Right],
                vec![Up],
                vec![Up, Right],
                vec![Up, Right, Right],
                vec![Up, Up],
                vec![Up, Up, Right],
                vec![Up, Up, Right, Right],
                vec![Right, Right, Down],
            ],
            // 2
            [
                vec![Down],
                vec![Left],
                vec![],
                vec![Right],
                vec![Left, Up],
                vec![Up],
                vec![Up, Right],
                vec![Left, Up, Up],
                vec![Up, Up],
                vec![Up, Up, Right],
                vec![Down, Right],
            ],
            // 3
            [
                vec![Left, Down],
                vec![Left, Left],
                vec![Left],
                vec![],
                vec![Left, Left, Up],
                vec![Left, Up],
                vec![Up],
                vec![Left, Left, Up, Up],
                vec![Left, Up, Up],
                vec![Up, Up],
                vec![Down],
            ],
            // 4
            [
                vec![Right, Down, Down],
                vec![Down],
                vec![Down, Right],
                vec![Down, Right, Right],
                vec![],
                vec![Right],
                vec![Right, Right],
                vec![Up],
                vec![Up, Right],
                vec![Up, Right, Right],
                vec![Right, Right, Down, Down],
            ],
            // 5
            [
                vec![Down, Down],
                vec![Left, Down],
                vec![Down],
                vec![Down, Right],
                vec![Left],
                vec![],
                vec![Right],
                vec![Left, Up],
                vec![Up],
                vec![Up, Right],
                vec![Down, Down, Right],
            ],
            // 6
            [
                vec![Left, Down, Down],
                vec![Left, Left, Down],
                vec![Left, Down],
                vec![Down],
                vec![Left, Left],
                vec![Left],
                vec![],
                vec![Left, Left, Up],
                vec![Left, Up],
                vec![Up],
                vec![Down, Down],
            ],
            // 7
            [
                vec![Right, Down, Down, Down],
                vec![Down, Down],
                vec![Down, Down, Right],
                vec![Down, Down, Right, Right],
                vec![Down],
                vec![Down, Right],
                vec![Down, Right, Right],
                vec![],
                vec![Right],
                vec![Right, Right],
                vec![Right, Right, Down, Down, Down],
            ],
            // 8
            [
                vec![Down, Down, Down],
                vec![Left, Down, Down],
                vec![Down, Down],
                vec![Down, Down, Right],
                vec![Left, Down],
                vec![Down],
                vec![Down, Right],
                vec![Left],
                vec![],
                vec![Right],
                vec![Down, Down, Down, Right],
            ],
            // 9
            [
                vec![Left, Down, Down, Down],
                vec![Left, Left, Down, Down],
                vec![Left, Down, Down],
                vec![Down, Down],
                vec![Left, Left, Down],
                vec![Left, Down],
                vec![Down],
                vec![Left, Left],
                vec![Left],
                vec![],
                vec![Down, Down, Down],
            ],
            // A
            [
                vec![Left],
                vec![Up, Left, Left],
                vec![Left, Up],
                vec![Up],
                vec![Up, Up, Left, Left],
                vec![Left, Up, Up],
                vec![Up, Up],
                vec![Up, Up, Up, Left, Left],
                vec![Left, Up, Up, Up],
                vec![Up, Up, Up],
                vec![],
            ],
        ];
        paths[*self as usize][*other as usize].clone()
    }
}

impl From<char> for Keypad {
    fn from(value: char) -> Self {
        match value {
            '0' => Self::Zero,
            '1' => Self::One,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'A' => Self::A,
            _ => unreachable!(),
        }
    }
}

impl Display for Keypad {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Keypad::Zero => '0',
                Keypad::One => '1',
                Keypad::Two => '2',
                Keypad::Three => '3',
                Keypad::Four => '4',
                Keypad::Five => '5',
                Keypad::Six => '6',
                Keypad::Seven => '7',
                Keypad::Eight => '8',
                Keypad::Nine => '9',
                Keypad::A => 'A',
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum RobotKeypad {
    A,
    Up,
    Right,
    Down,
    Left,
}

impl RobotKeypad {
    // ^ >
    // V
    // <
    fn optimal_path(&self, other: &Self) -> Vec<Self> {
        use RobotKeypad::*;

        let paths: [[Vec<Self>; 5]; 5] = [
            // A
            [
                vec![],
                vec![Left],
                vec![Down],
                vec![Left, Down],
                vec![Down, Left, Left],
            ],
            // ^
            [
                vec![Right],
                vec![],
                vec![Down, Right],
                vec![Down],
                vec![Down, Left],
            ],
            // >
            [
                vec![Up],
                vec![Left, Up],
                vec![],
                vec![Left],
                vec![Left, Left],
            ],
            // v
            [vec![Up, Right], vec![Up], vec![Right], vec![], vec![Left]],
            // <
            [
                vec![Right, Right, Up],
                vec![Right, Up],
                vec![Right, Right],
                vec![Right],
                vec![],
            ],
        ];
        let mut x = paths[*self as usize][*other as usize].clone();
        x.push(Self::A);
        x
    }
}

impl Display for RobotKeypad {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                RobotKeypad::Up => '^',
                RobotKeypad::Down => 'v',
                RobotKeypad::Right => '>',
                RobotKeypad::Left => '<',
                RobotKeypad::A => 'A',
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn robot_to_robot(code: &[RobotKeypad]) -> Vec<RobotKeypad> {
        let mut cur_position = RobotKeypad::A;
        let mut path = vec![];
        for &button in code {
            path.extend(cur_position.optimal_path(&button));
            cur_position = button;
        }
        path
    }

    #[test]
    fn part_1_test_1() {
        let input = &load_input("example")[0];
        let i_1 = keypad_to_robot(input);
        assert_eq!(
            i_1,
            [
                RobotKeypad::Left,
                RobotKeypad::A,
                RobotKeypad::Up,
                RobotKeypad::A,
                RobotKeypad::Up,
                RobotKeypad::Up,
                RobotKeypad::Right,
                RobotKeypad::A,
                RobotKeypad::Down,
                RobotKeypad::Down,
                RobotKeypad::Down,
                RobotKeypad::A
            ]
        );
        let i_2 = robot_to_robot(&i_1);
        assert_eq!(
            i_2,
            [
                RobotKeypad::Down,
                RobotKeypad::Left,
                RobotKeypad::Left,
                RobotKeypad::A,
                RobotKeypad::Right,
                RobotKeypad::Right,
                RobotKeypad::Up,
                RobotKeypad::A,
                RobotKeypad::Left,
                RobotKeypad::A,
                RobotKeypad::Right,
                RobotKeypad::A,
                RobotKeypad::Left,
                RobotKeypad::A,
                RobotKeypad::A,
                RobotKeypad::Down,
                RobotKeypad::Right,
                RobotKeypad::A,
                RobotKeypad::Up,
                RobotKeypad::A,
                RobotKeypad::Left,
                RobotKeypad::Down,
                RobotKeypad::A,
                RobotKeypad::A,
                RobotKeypad::A,
                RobotKeypad::Up,
                RobotKeypad::Right,
                RobotKeypad::A
            ]
        );
        let i_3 = robot_to_robot(&i_2);
        assert_eq!(
            i_3,
            [
                RobotKeypad::Left,
                RobotKeypad::Down,
                RobotKeypad::A,
                RobotKeypad::Left,
                RobotKeypad::A,
                RobotKeypad::A,
                RobotKeypad::Right,
                RobotKeypad::Right,
                RobotKeypad::Up,
                RobotKeypad::A,
                RobotKeypad::Down,
                RobotKeypad::A,
                RobotKeypad::A,
                RobotKeypad::Left,
                RobotKeypad::Up,
                RobotKeypad::A,
                RobotKeypad::Right,
                RobotKeypad::A,
                RobotKeypad::Down,
                RobotKeypad::Left,
                RobotKeypad::Left,
                RobotKeypad::A,
                RobotKeypad::Right,
                RobotKeypad::Right,
                RobotKeypad::Up,
                RobotKeypad::A,
                RobotKeypad::Down,
                RobotKeypad::A,
                RobotKeypad::Up,
                RobotKeypad::A,
                RobotKeypad::Down,
                RobotKeypad::Left,
                RobotKeypad::Left,
                RobotKeypad::A,
                RobotKeypad::Right,
                RobotKeypad::Right,
                RobotKeypad::Up,
                RobotKeypad::A,
                RobotKeypad::A,
                RobotKeypad::Left,
                RobotKeypad::Down,
                RobotKeypad::A,
                RobotKeypad::Right,
                RobotKeypad::A,
                RobotKeypad::Up,
                RobotKeypad::A,
                RobotKeypad::Left,
                RobotKeypad::A,
                RobotKeypad::Right,
                RobotKeypad::A,
                RobotKeypad::Down,
                RobotKeypad::Left,
                RobotKeypad::Left,
                RobotKeypad::A,
                RobotKeypad::Right,
                RobotKeypad::A,
                RobotKeypad::Up,
                RobotKeypad::Right,
                RobotKeypad::A,
                RobotKeypad::A,
                RobotKeypad::A,
                RobotKeypad::Left,
                RobotKeypad::A,
                RobotKeypad::Down,
                RobotKeypad::Right,
                RobotKeypad::A,
                RobotKeypad::Up,
                RobotKeypad::A
            ]
        )
    }

    #[test]
    fn part_1_test_2() {
        let input = load_input("example2");
        assert_eq!(solve(&input[0..1], 2), 68 * 29);
        assert_eq!(solve(&input[1..2], 2), 60 * 980);
        assert_eq!(solve(&input[2..3], 2), 68 * 179);
        assert_eq!(solve(&input[3..4], 2), 64 * 456);
        assert_eq!(solve(&input[4..5], 2), 64 * 379);
    }
    #[test]
    fn part_2_test_1() {
        let input = load_input("example");
        assert_eq!(solve(&input, 25), 2379451789590);
    }

    #[test]
    fn part_2_test_2() {
        let input = load_input("example2");
        assert_eq!(solve(&input, 25), 154115708116294);
    }
}
