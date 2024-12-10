use std::collections::VecDeque;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

fn main() {
    let mut input = load_input();
    for n in 0..50 {
        input.enchance();
        if n == 1 {
            println!("Solution for part 1: {}", input.get_lit());
        }
    }
    println!("Solution for part 2: {}", input.get_lit());
}

fn pixels_to_decimal(input: &[Pixel]) -> usize {
    let mut binary = String::new();
    for p in input {
        match p {
            Pixel::Black => binary.push('0'),
            Pixel::White => binary.push('1'),
        }
    }
    usize::from_str_radix(&binary, 2).unwrap()
}

fn load_input() -> Image {
    let file = File::open("input").expect("No input file found");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let algorithm: Vec<Pixel> = lines
        .next()
        .unwrap()
        .unwrap()
        .split("")
        .flat_map(Pixel::from_str)
        .collect();
    let mut image: VecDeque<VecDeque<Pixel>> = VecDeque::new();
    lines.next();
    for line in lines {
        let row: VecDeque<Pixel> = line.unwrap().chars().flat_map(Pixel::try_from).collect();
        image.push_back(row);
    }
    Image::new(image, algorithm)
}

#[derive(Debug, Clone, Copy)]
enum Pixel {
    Black,
    White,
}

impl Pixel {
    fn as_char(&self) -> char {
        match self {
            Pixel::Black => '.',
            Pixel::White => '#',
        }
    }
}

impl FromStr for Pixel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Self::Black),
            "#" => Ok(Self::White),
            _ => Err(()),
        }
    }
}

impl TryFrom<char> for Pixel {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Self::Black),
            '#' => Ok(Self::White),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Image {
    algorithm: Vec<Pixel>,
    background: Pixel,
    image: VecDeque<VecDeque<Pixel>>,
}

impl Image {
    fn new(image: VecDeque<VecDeque<Pixel>>, algo: Vec<Pixel>) -> Self {
        let mut new = Self {
            algorithm: algo,
            background: Pixel::Black,
            image,
        };
        new.add_border();
        new
    }

    // Adds a border around the image that's made out of 2 background pixels. The border is used to simulate the infinite size
    fn add_border(&mut self) {
        for row in self.image.iter_mut() {
            for _ in 0..2 {
                row.push_front(self.background);
                row.push_back(self.background);
            }
        }
        let columns = self.image[0].len();
        for _ in 0..2 {
            self.image
                .push_front(VecDeque::from(vec![self.background; columns]));
            self.image
                .push_back(VecDeque::from(vec![self.background; columns]));
        }
    }

    fn enchance(&mut self) {
        // Change the background value.
        // 9 Black = 0
        // 9 White = 511
        self.background = match self.background {
            Pixel::Black => self.algorithm[0],
            Pixel::White => self.algorithm[511],
        };
        let mut new_image: VecDeque<VecDeque<Pixel>> = VecDeque::new();
        for y in 1..self.image.len() - 1 {
            let mut row: VecDeque<Pixel> = VecDeque::new();
            for x in 1..self.image[0].len() - 1 {
                let num = pixels_to_decimal(&self.get_neighbours(x, y));
                row.push_back(self.algorithm[num]);
            }
            new_image.push_back(row);
        }
        self.image = new_image;
        self.add_border();
    }

    fn get_neighbours(&self, x: usize, y: usize) -> Vec<Pixel> {
        let mut output = String::new();
        for row in y - 1..=y + 1 {
            for column in x - 1..=x + 1 {
                output.push(self.image[row][column].as_char());
            }
        }
        output.chars().flat_map(Pixel::try_from).collect()
    }

    fn get_lit(&self) -> usize {
        let mut counter = 0;
        for row in &self.image {
            for pixel in row {
                if let Pixel::White = pixel {
                    counter += 1;
                }
            }
        }
        counter
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for y in 2..self.image.len() - 2 {
            for x in 2..self.image[0].len() - 2 {
                output.push(self.image[y][x].as_char());
            }
            output.push('\n');
        }
        write!(f, "{}", output)
    }
}

#[test]
fn algorithm_len() {
    let image = load_input();
    assert_eq!(image.algorithm.len(), 512)
}
