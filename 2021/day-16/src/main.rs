use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

fn main() {
    let input = load_input();
    println!("Solution for part 1: {}", input.count_versions());
    println!("Solution for part 2: {}", input.calculate_value());
}

fn load_input() -> Packet {
    let file = File::open("input").expect("No input file found");
    let reader = BufReader::new(file);
    let line = reader.lines().next().unwrap().unwrap();
    let binary = convert_hex_to_binary(&line);
    Packet::from_str(&binary).unwrap()
}

fn convert_hex_to_binary(s: &str) -> String {
    let mut binary = String::new();
    for c in s.chars() {
        binary.push_str(match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => unreachable!(),
        })
    }
    binary
}

#[derive(Debug)]
struct Packet {
    version: usize,
    id: usize,
    packet_type: PacketType,
    packet_length: usize,
}

impl Packet {
    fn count_versions(&self) -> usize {
        match &self.packet_type {
            PacketType::Literal(_) => self.version,
            PacketType::Operator(sub_packets) => {
                let mut version_counter = self.version;
                for sub_packet in sub_packets {
                    version_counter += sub_packet.count_versions();
                }
                version_counter
            }
        }
    }

    fn calculate_value(&self) -> usize {
        match &self.packet_type {
            PacketType::Literal(value) => *value,
            PacketType::Operator(sub_packets) => match self.id {
                0 => {
                    let mut counter = 0;
                    for sub_packet in sub_packets {
                        counter += sub_packet.calculate_value();
                    }
                    counter
                }
                1 => {
                    if sub_packets.len() == 1 {
                        sub_packets[0].calculate_value()
                    } else {
                        let mut counter = 1;
                        for sub_packet in sub_packets {
                            counter *= sub_packet.calculate_value();
                        }
                        counter
                    }
                }
                2 => sub_packets
                    .iter()
                    .map(|x| x.calculate_value())
                    .min()
                    .unwrap(),
                3 => sub_packets
                    .iter()
                    .map(|x| x.calculate_value())
                    .max()
                    .unwrap(),
                5 => {
                    let value_first = sub_packets[0].calculate_value();
                    let value_second = sub_packets[1].calculate_value();
                    if value_first > value_second {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    let value_first = sub_packets[0].calculate_value();
                    let value_second = sub_packets[1].calculate_value();
                    if value_first < value_second {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    let value_first = sub_packets[0].calculate_value();
                    let value_second = sub_packets[1].calculate_value();
                    if value_first == value_second {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            },
        }
    }
}

impl FromStr for Packet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let version = usize::from_str_radix(&s[0..3], 2).unwrap();
        let id = usize::from_str_radix(&s[3..6], 2).unwrap();
        let (packet, length) = PacketType::new(&id, &s[6..]);
        Ok(Self {
            version,
            id,
            packet_type: packet,
            packet_length: length,
        })
    }
}

#[derive(Debug)]
enum PacketType {
    Literal(usize),
    Operator(Vec<Packet>),
}

impl PacketType {
    /// Creates a new PacketType and returns it's own packet length
    ///
    /// The length is returned because other Operator variants need to know the length of it's sub packets.
    fn new(id: &usize, data: &str) -> (Self, usize) {
        match *id {
            4 => {
                let mut binary = String::new();
                let mut iter = data.chars();
                let mut iter_counter = 0;
                loop {
                    iter_counter += 1;
                    let mut last = false;
                    if iter.next().unwrap() == '0' {
                        last = true;
                    }
                    for _ in 0..4 {
                        binary.push(iter.next().unwrap())
                    }
                    if last {
                        break;
                    }
                }
                (
                    Self::Literal(usize::from_str_radix(&binary, 2).unwrap()),
                    6 + (iter_counter * 5),
                )
            }
            _ => {
                let mut packet_length_counter = 6;
                let (length_type, length) = match data.chars().next().unwrap() {
                    '0' => (
                        LengthType::Zero,
                        usize::from_str_radix(&data[1..16], 2).unwrap(),
                    ),
                    '1' => (
                        LengthType::One,
                        usize::from_str_radix(&data[1..12], 2).unwrap(),
                    ),
                    _ => unreachable!(),
                };
                let mut sub_packets = Vec::new();
                match length_type {
                    LengthType::Zero => {
                        packet_length_counter += 16;
                        let mut length_counter = 0;
                        let end_len = 16 + length;
                        while length_counter < length {
                            let sub_packet =
                                Packet::from_str(&data[16 + length_counter..=end_len]).unwrap();
                            length_counter += sub_packet.packet_length;
                            packet_length_counter += sub_packet.packet_length;
                            sub_packets.push(sub_packet);
                        }
                    }
                    LengthType::One => {
                        let mut length_counter = 12;
                        packet_length_counter += 12;
                        while sub_packets.len() != length {
                            let sub_packet = Packet::from_str(&data[length_counter..]).unwrap();
                            length_counter += sub_packet.packet_length;
                            packet_length_counter += sub_packet.packet_length;
                            sub_packets.push(sub_packet);
                        }
                    }
                }
                (Self::Operator(sub_packets), packet_length_counter)
            }
        }
    }
}

#[derive(Debug)]
enum LengthType {
    Zero,
    One,
}
