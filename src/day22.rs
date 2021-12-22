use std::cmp::{max, min};
use std::collections::HashSet;
use aoc_runner_derive::{aoc, aoc_generator};
use std::ops::RangeInclusive;
use std::str::FromStr;
use itertools::Itertools;
use regex::Regex;
use crate::day22::Day22Error::InvalidInstruction;

#[derive(Debug)]
enum Day22Error {
    InvalidInstruction,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Instruction {
    On,
    Off,
}

impl FromStr for Instruction {
    type Err = Day22Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(Instruction::On),
            "off" => Ok(Instruction::Off),
            _ => Err(InvalidInstruction),
        }
    }
}

#[aoc_generator(day22)]
fn parse(input: &str) -> Vec<(Instruction, RangeInclusive<i32>, RangeInclusive<i32>, RangeInclusive<i32>)> {
    let re = Regex::new(r"(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)").unwrap();

    input
        .lines()
        .filter_map(|line| {
            if let Some(c) = re.captures(line) {
                let instruction = Instruction::from_str(c.get(1).unwrap().as_str()).unwrap();
                let x = c.get(2).unwrap().as_str().parse::<i32>().unwrap()..=c.get(3).unwrap().as_str().parse::<i32>().unwrap();
                let y = c.get(4).unwrap().as_str().parse::<i32>().unwrap()..=c.get(5).unwrap().as_str().parse::<i32>().unwrap();
                let z = c.get(6).unwrap().as_str().parse::<i32>().unwrap()..=c.get(7).unwrap().as_str().parse::<i32>().unwrap();

                return Some((instruction, x, y, z));
            };

            None
        })
        .collect()
}

#[aoc(day22, part1)]
fn part1(instructions: &Vec<(Instruction, RangeInclusive<i32>, RangeInclusive<i32>, RangeInclusive<i32>)>) -> usize {
    let mut cubes: HashSet<(i32, i32, i32)> = HashSet::new();

    for (instruction, xr, yr, zr) in instructions {
        let xr = (max(*xr.start(), -50))..=(min(*xr.end(), 50));
        let yr = (max(*yr.start(), -50))..=(min(*yr.end(), 50));
        let zr = (max(*zr.start(), -50))..=(min(*zr.end(), 50));

        for p in [xr.clone(), yr.clone(), zr.clone()].iter().cloned().multi_cartesian_product() {
            let (x, y, z) = p.iter().copied().collect_tuple().unwrap();

            if (-50..50).contains(&x) && (-50..50).contains(&y) && (-50..50).contains(&z) {
                match instruction {
                    Instruction::On => { cubes.insert((x, y, z)); }
                    Instruction::Off => { cubes.remove(&(x, y, z)); }
                }
            }
        }
    }

    cubes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(39, part1(&parse(include_str!("../input/2021/day22.part1.test.39.txt"))));
    }
}