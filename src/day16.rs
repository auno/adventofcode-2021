use aoc_runner_derive::{aoc, aoc_generator};
use crate::day16::Packet::{LiteralValuePacket, OperatorPacket};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Operator {
    Sum,
    Product,
    Min,
    Max,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl From<&[u8]> for Operator {
    fn from(input: &[u8]) -> Self {
        debug_assert_eq!(input.len(), 3);
        match decimal(&input[0..3]) {
            0 => Operator::Sum,
            1 => Operator::Product,
            2 => Operator::Min,
            3 => Operator::Max,
            5 => Operator::GreaterThan,
            6 => Operator::LessThan,
            7 => Operator::EqualTo,
            _ => panic!()
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Packet {
    LiteralValuePacket { version: u8, value: u64 },
    OperatorPacket { version: u8, operator: Operator, packets: Vec<Packet> },
}

impl Packet {
    fn value(&self) -> u64 {
        match self {
            Packet::LiteralValuePacket { version: _, value } => *value,
            Packet::OperatorPacket { version: _, operator: Operator::Sum, packets } =>
                packets.iter().map(Self::value).sum(),
            Packet::OperatorPacket { version: _, operator: Operator::Product, packets } =>
                packets.iter().map(Self::value).product(),
            Packet::OperatorPacket { version: _, operator: Operator::Min, packets } =>
                packets.iter().map(Self::value).min().unwrap(),
            Packet::OperatorPacket { version: _, operator: Operator::Max, packets } =>
                packets.iter().map(Self::value).max().unwrap(),
            Packet::OperatorPacket { version: _, operator: Operator::GreaterThan, packets } =>
                match packets[0].value() > packets[1].value() {
                true => 1,
                false => 0,
            },
            Packet::OperatorPacket { version: _, operator: Operator::LessThan, packets } =>
                match packets[0].value() < packets[1].value() {
                    true => 1,
                    false => 0,
                },
            Packet::OperatorPacket { version: _, operator: Operator::EqualTo, packets } =>
                match packets[0].value() == packets[1].value() {
                    true => 1,
                    false => 0,
                },
        }
    }

    fn sum_version_numbers(&self) -> u64 {
        match self {
            Packet::LiteralValuePacket { version, value: _ } => *version as u64,
            Packet::OperatorPacket {  version, operator: _, packets  } =>
                *version as u64 + packets.iter().map(Self::sum_version_numbers).map(|a| a).sum::<u64>(),
        }
    }
}

fn decimal(input: &[u8]) -> u64 {
    debug_assert!(input.len() <= 64);
    input.iter().fold(0, |acc, value| (acc << 1) + *value as u64)
}

fn parse_literal_packet(input: &[u8], mut pos: usize) -> (Packet, usize) {
    let version = decimal(&input[pos..(pos+3)]) as u8;
    let mut value = vec![];
    pos += 6;

    loop {
        for bit in &input[(pos + 1)..(pos + 5)] {
            value.push(*bit);
        }

        pos += 5;

        if input[pos - 5] == 0 {
            break;
        }
    }

    let value = decimal(&value);

    (LiteralValuePacket { version, value }, pos)
}

fn parse_operator_packet(input: &[u8], mut pos: usize) -> (Packet, usize) {
    let version = decimal(&input[pos..(pos+3)]) as u8;
    let operator = (&input[(pos+3)..(pos+6)]).into();
    let mut packets = vec![];
    pos += 6;

    match input[pos] {
        0 => {
            pos += 1;
            let mut bits_to_read: usize = decimal(&input[pos..(pos+15)]) as usize;
            pos += 15;

            while bits_to_read > 0 {
                let (packet, new_pos) = parse_packet(input, pos);
                packets.push(packet);
                bits_to_read -= new_pos - pos;
                pos = new_pos;
            }

        },
        1 => {
            pos += 1;
            let packets_to_read = decimal(&input[pos..(pos+11)]) as usize;
            pos += 11;

            for _ in 0..packets_to_read {
                let (packet, new_pos) = parse_packet(input, pos);
                packets.push(packet);
                pos = new_pos;
            }
        },
        _ => panic!(),
    }

    (OperatorPacket { version, operator, packets }, pos)
}

fn parse_packet(input: &[u8], pos: usize) -> (Packet, usize) {
    match &input[(pos+3)..(pos+6)] {
        &[1, 0, 0] => parse_literal_packet(input, pos),
        _ => parse_operator_packet(input, pos),
    }
}

#[aoc_generator(day16)]
fn parse_input(input: &str) -> Vec<u8> {
    input
        .lines()
        .flat_map(|line| line.chars())
        .flat_map(|c| format!("{:04b}", c.to_digit(16).unwrap()).chars().collect::<Vec<_>>())
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

#[aoc(day16, part1)]
fn part1(transmission: &[u8]) -> u64 {
    let (packet, _) = parse_packet(transmission, 0);
    packet.sum_version_numbers()
}

#[aoc(day16, part2)]
fn part2(transmission: &[u8]) -> u64 {
    let (packet, _) = parse_packet(transmission, 0);
    packet.value()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(16, part1(&parse_input(include_str!("../input/2021/day16.part1.test.16.txt"))));
    }

    #[test]
    fn part1_example2() {
        assert_eq!(12, part1(&parse_input(include_str!("../input/2021/day16.part1.test.12.txt"))));
    }

    #[test]
    fn part1_example3() {
        assert_eq!(23, part1(&parse_input(include_str!("../input/2021/day16.part1.test.23.txt"))));
    }

    #[test]
    fn part1_example4() {
        assert_eq!(31, part1(&parse_input(include_str!("../input/2021/day16.part1.test.31.txt"))));
    }

    #[test]
    fn part1_example5() {
        assert_eq!(14, part1(&parse_input(include_str!("../input/2021/day16.part1.test.14.txt"))));
    }

    #[test]
    fn part1_example6() {
        assert_eq!(6, part1(&parse_input(include_str!("../input/2021/day16.part1.test.6.txt"))));
    }

    #[test]
    fn part1_example7() {
        assert_eq!(9, part1(&parse_input(include_str!("../input/2021/day16.part1.test.9.txt"))));
    }

    #[test]
    fn part2_example0() {
        assert_eq!(3, part2(&parse_input(include_str!("../input/2021/day16.part2.test.0.txt"))));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(54, part2(&parse_input(include_str!("../input/2021/day16.part2.test.1.txt"))));
    }

    #[test]
    fn part2_example2() {
        assert_eq!(7, part2(&parse_input(include_str!("../input/2021/day16.part2.test.2.txt"))));
    }

    #[test]
    fn part2_example3() {
        assert_eq!(9, part2(&parse_input(include_str!("../input/2021/day16.part2.test.3.txt"))));
    }

    #[test]
    fn part2_example4() {
        assert_eq!(1, part2(&parse_input(include_str!("../input/2021/day16.part2.test.4.txt"))));
    }

    #[test]
    fn part2_example5() {
        assert_eq!(0, part2(&parse_input(include_str!("../input/2021/day16.part2.test.5.txt"))));
    }

    #[test]
    fn part2_example6() {
        assert_eq!(0, part2(&parse_input(include_str!("../input/2021/day16.part2.test.6.txt"))));
    }

    #[test]
    fn part2_example7() {
        assert_eq!(1, part2(&parse_input(include_str!("../input/2021/day16.part2.test.7.txt"))));
    }
}