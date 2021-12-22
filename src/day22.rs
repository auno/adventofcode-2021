use std::cmp::{max, min};
use aoc_runner_derive::{aoc, aoc_generator};
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

type Cuboid = ((i64, i64), (i64, i64), (i64, i64));

#[aoc_generator(day22)]
fn parse(input: &str) -> Vec<(Instruction, Cuboid)> {
    let re = Regex::new(r"(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)").unwrap();

    input
        .lines()
        .filter_map(|line| {
            if let Some(c) = re.captures(line) {
                let instruction = Instruction::from_str(c.get(1).unwrap().as_str()).unwrap();
                let x = (c.get(2).unwrap().as_str().parse::<i64>().unwrap(), c.get(3).unwrap().as_str().parse::<i64>().unwrap() + 1);
                let y = (c.get(4).unwrap().as_str().parse::<i64>().unwrap(), c.get(5).unwrap().as_str().parse::<i64>().unwrap() + 1);
                let z = (c.get(6).unwrap().as_str().parse::<i64>().unwrap(), c.get(7).unwrap().as_str().parse::<i64>().unwrap() + 1);

                return Some((instruction, (x, y, z)));
            };

            None
        })
        .collect()
}

fn disjoint_range(a: (i64, i64), b: (i64, i64)) -> bool {
    !(a.0..a.1).contains(&b.0) && !(a.0..a.1).contains(&b.1) && !(b.0..b.1).contains(&a.0) && !(b.0..b.1).contains(&a.1)
}

fn empty((xr, yr, zr): Cuboid) -> bool {
    !(xr.1 > xr.0 && yr.1 > yr.0 && zr.1 > zr.0)
}

fn deoverlap((breaker_xr, breaker_yr, breaker_zr): Cuboid, (breakee_xr, breakee_yr, breakee_zr): Cuboid) -> Vec<Cuboid> {
    if disjoint_range(breaker_xr, breakee_xr) || disjoint_range(breaker_yr, breakee_yr) || disjoint_range(breaker_zr, breakee_zr) {
        return vec![(breakee_xr, breakee_yr, breakee_zr)];
    }

    [breaker_xr, breaker_yr, breaker_zr].into_iter().zip([breakee_xr, breakee_yr, breakee_zr]
        .into_iter())
        .map(|(breaker_range, breakee_range)| vec![
            (breakee_range.0, min(breaker_range.0, breakee_range.1)),
            (max(breaker_range.0, breakee_range.0), min(breaker_range.1, breakee_range.1)),
            (max(breaker_range.1, breakee_range.0), breakee_range.1),
        ])
        .multi_cartesian_product()
        .map(|ranges| (ranges[0], ranges[1], ranges[2]))
        .filter(|cuboid| !empty(*cuboid))
        .filter(|cuboid| *cuboid != (
            (max(breaker_xr.0, breakee_xr.0), min(breaker_xr.1, breakee_xr.1)),
            (max(breaker_yr.0, breakee_yr.0), min(breaker_yr.1, breakee_yr.1)),
            (max(breaker_zr.0, breakee_zr.0), min(breaker_zr.1, breakee_zr.1)),
        ))
        .collect()
}

fn resolve(instructions: &Vec<(Instruction, Cuboid)>) -> Vec<Cuboid> {
    let mut inclusions: Vec<Cuboid> = vec![];

    for &(instruction, cuboid) in instructions {
        inclusions = inclusions.iter()
            .flat_map(|inclusion| deoverlap(cuboid, *inclusion))
            .collect();

        if instruction == Instruction::On {
            inclusions.push(cuboid)
        }
    }

    inclusions
}

#[aoc(day22, part1)]
fn part1(instructions: &Vec<(Instruction, Cuboid)>) -> i64 {
    let instructions = instructions
        .iter()
        .map(|&(instruction, (xr, yr, zr))| {
            let xr = (max(xr.0, -50), min(xr.1, 51));
            let yr = (max(yr.0, -50), min(yr.1, 51));
            let zr = (max(zr.0, -50), min(zr.1, 51));

            (instruction, (xr, yr, zr))
        })
        .filter(|&(_, cuboid)| !empty(cuboid))
        .collect();

    resolve(&instructions)
        .iter()
        .map(|&(xr, yr, zr)| (xr.1 - xr.0) * (yr.1 - yr.0) * (zr.1 - zr.0))
        .sum()
}

#[aoc(day22, part2)]
fn part2(instructions: &Vec<(Instruction, Cuboid)>) -> i64 {
    resolve(instructions)
        .iter()
        .map(|&(xr, yr, zr)| (xr.1 - xr.0) * (yr.1 - yr.0) * (zr.1 - zr.0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cuboid_to_points((xr, yr, zr): Cuboid) -> Vec<(i64, i64, i64)> {
        [xr.0..xr.1, yr.0..yr.1, zr.0..zr.1].into_iter()
            .multi_cartesian_product()
            .map(|coords| (coords[0], coords[1], coords[2]))
            .collect()
    }

    #[test]
    fn part1_example1() {
        assert_eq!(39, part1(&parse(include_str!("../input/2021/day22.part1.test.39.txt"))));
    }

    #[test]
    fn part1_example2() {
        assert_eq!(590784, part1(&parse(include_str!("../input/2021/day22.part1.test.590784.txt"))));
    }

    #[test]
    fn part2_example() {
        assert_eq!(2758514936282235, part2(&parse(include_str!("../input/2021/day22.part2.test.2758514936282235.txt"))));
    }

    #[test]
    fn deoverlap_test1() {
        let breakee = ((0, 3), (0, 3), (0, 3));
        let breaker = ((1, 2), (1, 2), (1, 2));
        let expected: Vec<(i64, i64, i64)> = cuboid_to_points(breakee)
            .into_iter()
            .filter(|coords| *coords != (1, 1, 1))
            .collect();

        let actual: Vec<(i64, i64, i64)> = deoverlap(breaker, breakee)
            .into_iter()
            .flat_map(|cuboid| cuboid_to_points(cuboid))
            .collect();

        assert_eq!(26, actual.len());
        assert_eq!(expected, actual);
    }

    #[test]
    fn deoverlap_test2() {
        let breakee = ((0, 3), (0, 3), (0, 3));
        let breaker = ((2, 3), (2, 3), (2, 3));
        let expected: Vec<(i64, i64, i64)> = cuboid_to_points(breakee)
            .into_iter()
            .filter(|coords| *coords != (2, 2, 2))
            .sorted()
            .collect();

        let actual: Vec<(i64, i64, i64)> = deoverlap(breaker, breakee)
            .into_iter()
            .flat_map(|cuboid| cuboid_to_points(cuboid))
            .sorted()
            .collect();

        assert_eq!(26, actual.len());
        assert_eq!(expected, actual);
    }

    #[test]
    fn deoverlap_test3() {
        let breakee = ((0, 3), (0, 3), (0, 3));
        let breaker = ((4, 5), (4, 5), (4, 5));
        let expected: Vec<(i64, i64, i64)> = cuboid_to_points(breakee)
            .into_iter()
            .sorted()
            .collect();

        let actual: Vec<(i64, i64, i64)> = deoverlap(breaker, breakee)
            .into_iter()
            .flat_map(|cuboid| cuboid_to_points(cuboid))
            .sorted()
            .collect();

        assert_eq!(27, actual.len());
        assert_eq!(expected, actual);
    }

    #[test]
    fn deoverlap_test4() {
        let breakee = ((0, 3), (0, 3), (0, 3));
        let breaker = ((2, 5), (2, 5), (2, 5));
        let expected: Vec<(i64, i64, i64)> = cuboid_to_points(breakee)
            .into_iter()
            .filter(|coords| *coords != (2, 2, 2))
            .sorted()
            .collect();

        let actual: Vec<(i64, i64, i64)> = deoverlap(breaker, breakee)
            .into_iter()
            .flat_map(|cuboid| cuboid_to_points(cuboid))
            .sorted()
            .collect();

        assert_eq!(26, actual.len());
        assert_eq!(expected, actual);
    }

    #[test]
    fn deoverlap_test5() {
        let breakee = ((0, 3), (0, 3), (0, 3));
        let breaker = ((-4, 1), (-4, 1), (-4, 1));
        let expected: Vec<(i64, i64, i64)> = cuboid_to_points(breakee)
            .into_iter()
            .filter(|coords| *coords != (0, 0, 0))
            .sorted()
            .collect();

        let actual: Vec<(i64, i64, i64)> = deoverlap(breaker, breakee)
            .into_iter()
            .flat_map(|cuboid| cuboid_to_points(cuboid))
            .sorted()
            .collect();

        assert_eq!(26, actual.len());
        assert_eq!(expected, actual);
    }
}