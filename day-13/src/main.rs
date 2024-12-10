use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input = load_input("input");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &[Element]) -> usize {
    let mut counter = 0;
    for (i, pair) in input.chunks(2).enumerate() {
        let first = &pair[0];
        let second = &pair[1];
        let x = compare(first, second);
        match x {
            Ordering::Less => {
                counter += i + 1;
            }
            Ordering::Equal => {
                panic!("There shouldn't be equal packets")
            }
            Ordering::Greater => {}
        }
    }
    counter
}

fn part_2(mut input: Vec<Element>) -> usize {
    let two = Element::List(vec![Element::List(vec![Element::Int(2)])]);
    let six = Element::List(vec![Element::List(vec![Element::Int(6)])]);
    input.push(two.clone());
    input.push(six.clone());
    input.sort_unstable_by(compare);
    let mut counter = 1;
    for (i, ele) in input.iter().enumerate() {
        if ele == &two || ele == &six {
            counter *= i + 1;
        }
    }
    counter
}

/// Compares 2 elements
fn compare(first_ele: &Element, second_ele: &Element) -> Ordering {
    match (first_ele, second_ele) {
        // Compare 2 numbers
        (Element::Int(num_1), Element::Int(num_2)) => num_1.cmp(num_2),
        // Compare 2 lists
        (Element::List(first_list), Element::List(second_list)) => {
            // Iterate over each element in both lists
            for (first, second) in first_list.iter().zip(second_list.iter()) {
                match compare(first, second) {
                    // If left is less you can safely exit
                    Ordering::Less => {
                        return Ordering::Less;
                    }
                    // If they are equal keep iterating
                    Ordering::Equal => {}
                    // If left is greater you can safely exit
                    Ordering::Greater => {
                        return Ordering::Greater;
                    }
                }
            }
            // Only reached if all elements where equal
            // Compare length of lists
            first_list.len().cmp(&second_list.len())
        }
        // Number + List
        // Turns number into a Element::List with a single number and compares them
        (Element::Int(num), list @ Element::List(_)) => {
            let new_ele = Element::List(vec![Element::Int(*num)]);
            compare(&new_ele, list)
        }

        // List + Number
        // Same as above but other way around
        (list @ Element::List(_), Element::Int(num)) => {
            let new_ele = Element::List(vec![Element::Int(*num)]);
            compare(list, &new_ele)
        }
    }
}

fn load_input(name: &str) -> Vec<Element> {
    let file = File::open(name).expect("No input file found");
    let reader = BufReader::new(file);
    let mut packets: Vec<Element> = Vec::new();
    for line in reader.lines().map(|l| l.unwrap()) {
        if line.is_empty() {
            continue;
        }
        packets.push(parse_line(line));
    }
    packets
}

/// Just a small wrapper that starts the parsing
fn parse_line(s: String) -> Element {
    let chars: Vec<char> = s.chars().collect();
    let (l, _) = parse_list(&chars, 1);
    l
}

/// Will parse the given list.
///
/// Will add every number it finds. Returns if it finds the end of the list, calls itself recursively if a new list found.
fn parse_list(input: &[char], mut i: usize) -> (Element, usize) {
    let mut cur_list: Vec<Element> = Vec::new();
    // The currently found digits
    let mut cur_num: Vec<char> = Vec::new();
    while i < input.len() {
        match input[i] {
            // End of list
            ']' => {
                // Need to add the number that has been stored before breaking
                if !cur_num.is_empty() {
                    let num: u64 = String::from_iter(cur_num).parse().unwrap();
                    cur_list.push(Element::Int(num));
                }
                break;
            }
            // End of number
            ',' => {
                // Required because a comma appears after end of list
                if !cur_num.is_empty() {
                    let num: u64 = String::from_iter(cur_num.drain(..)).parse().unwrap();
                    cur_list.push(Element::Int(num));
                }
            }
            // New list start
            '[' => {
                let (nested_list, x) = parse_list(input, i + 1);
                // i needs to be changed to skip the nested list
                i = x;
                cur_list.push(nested_list);
            }
            // Digit
            c => {
                cur_num.push(c);
            }
        }
        i += 1;
    }
    (Element::List(cur_list), i)
}

#[derive(Debug, Clone, PartialEq)]
enum Element {
    Int(u64),
    List(Vec<Element>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = load_input("example");
        assert_eq!(part_1(&input), 13);
    }

    #[test]
    fn compare_numbers() {
        let bigger = Element::Int(5);
        let smaller = Element::Int(3);
        assert_eq!(compare(&smaller, &bigger), Ordering::Less);
        assert_eq!(compare(&bigger, &bigger), Ordering::Equal);
        assert_eq!(compare(&bigger, &smaller), Ordering::Greater);
    }

    #[test]
    fn compare_diff_len_lists() {
        let shorter = Element::List(vec![
            Element::Int(1),
            Element::Int(2),
            Element::Int(3),
            Element::Int(4),
        ]);
        let longer = Element::List(vec![
            Element::Int(1),
            Element::Int(2),
            Element::Int(3),
            Element::Int(4),
            Element::Int(5),
        ]);
        assert_eq!(compare(&shorter, &longer), Ordering::Less);
        assert_eq!(compare(&longer, &longer), Ordering::Equal);
        assert_eq!(compare(&longer, &shorter), Ordering::Greater);
    }
}
