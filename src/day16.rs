use aoc_runner_derive::{aoc, aoc_generator};
use crate::day16::Packet::{LiteralValuePacket, OperatorPacket};

#[derive(Debug, Clone, Eq, PartialEq)]
enum Packet {
    LiteralValuePacket { version: u8, value: u32 },
    OperatorPacket { version: u8, packets: Vec<Packet> },
}

fn parse_literal_packet(input: &[u8], mut pos: usize) -> (Packet, usize) {
    let version = input[pos..(pos+3)].iter().fold(0, |acc, value| (acc << 1) + value);
    let mut bits = vec![];
    pos += 6;

    loop {
        for bit in &input[(pos + 1)..(pos + 5)] {
            bits.push(bit);
        }

        pos += 5;

        if input[pos - 5] == 0 {
            break;
        }
    }

    let value = bits.iter().fold(0, |acc, v| (acc << 1) + **v as u32);

    (LiteralValuePacket { version, value }, pos)
}

fn parse_operator_packet(input: &[u8], mut pos: usize) -> (Packet, usize) {
    let version = input[pos..(pos+3)].iter().fold(0, |acc, value| (acc << 1) + value);
    let mut packets = vec![];
    pos += 6;

    match input[pos] {
        0 => {
            pos += 1;
            let mut bits_to_read: usize = input[pos..(pos+15)].iter().fold(0, |acc, value| (acc << 1) + *value as usize);
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
            let packets_to_read = input[pos..(pos+11)].iter().fold(0, |acc, value| (acc << 1) + *value as usize);
            pos += 11;

            for _ in 0..packets_to_read {
                let (packet, new_pos) = parse_packet(input, pos);
                packets.push(packet);
                pos = new_pos;
            }
        },
        _ => panic!(),
    }

    (OperatorPacket { version, packets }, pos)
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

fn sum_version_numbers(packet: &Packet) -> u32 {
    match packet {
        Packet::LiteralValuePacket { version, value: _ } => *version as u32,
        Packet::OperatorPacket {  version, packets  } =>
            *version as u32 + packets.iter().map(sum_version_numbers).map(|a| a).sum::<u32>(),
    }
}

#[aoc(day16, part1)]
fn part1(transmission: &[u8]) -> u32 {
    let (packet, _) = parse_packet(transmission, 0);

    sum_version_numbers(&packet)
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
}