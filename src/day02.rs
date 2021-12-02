use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;
use std::error::Error;
use core::fmt::Display;
use std::fmt::Formatter;

use Movement::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Movement {
    Up(i32),
    Down(i32),
    Forward(i32),
}

#[derive(Debug)]
pub enum MovementParseError {
    InvalidFormat
}

impl Display for MovementParseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            MovementParseError::InvalidFormat => { write!(f, "InvalidFormat") }
        }
    }
}

impl Error for MovementParseError {}

impl FromStr for Movement {
    type Err = MovementParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(' ') {
            Some(("up", amount)) => Ok(Up(amount.parse().unwrap())),
            Some(("down", amount)) => Ok(Down(amount.parse().unwrap())),
            Some(("forward", amount)) => Ok(Forward(amount.parse().unwrap())),
            _ => Err(MovementParseError::InvalidFormat)
        }
    }
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Result<Vec<Movement>, MovementParseError> {
    input
        .lines()
        .map(|line| line.parse())
        .collect()
}

#[aoc(day2, part1)]
fn part1(movements: &[Movement]) -> i32 {
    let (depth, forward): (i32, i32) = movements.iter()
        .fold((0, 0), |(depth, forward), movement| {
            match movement {
                Up(amount) => (depth - amount, forward),
                Down(amount) => (depth + amount, forward),
                Forward(amount) => (depth, forward + amount),
            }
        });

    depth * forward
}

#[aoc(day2, part2)]
fn part2(movements: &[Movement]) -> i32 {
    let (depth, forward, _): (i32, i32, i32) = movements.iter()
        .fold((0, 0, 0), |(depth, forward, aim), movement| {
            match movement {
                Up(amount) => (depth, forward, aim - amount),
                Down(amount) => (depth, forward, aim + amount),
                Forward(amount) => (depth + aim * amount, forward + amount, aim),
            }
        });

    depth * forward
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(150, part1(&parse(include_str!("../input/2021/day2.part1.test.150.txt")).unwrap()));
    }

    #[test]
    fn part2_example() {
        assert_eq!(900, part2(&parse(include_str!("../input/2021/day2.part2.test.900.txt")).unwrap()));
    }
}