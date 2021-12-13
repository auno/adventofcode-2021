use std::collections::HashSet;
use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day13)]
fn parse(input: &str) -> (HashSet<(i32, i32)>, Vec<(String, i32)>) {
    let coords: HashSet<(i32, i32)> = input
        .lines()
        .filter(|line| line.contains(","))
        .map(|line| {
            let mut parts = line.split(",");
            (parts.next().unwrap().parse().unwrap(), parts.next().unwrap().parse().unwrap())
        })
        .collect();

    let re = Regex::new(r"^fold along ([xy])=(\d+)$").unwrap();
    let folds: Vec<(String, i32)> = input.lines()
        .filter_map(|line| {
            match re.captures(line) {
                None => None,
                Some(c) => {

                    Some((c.get(1).unwrap().as_str().to_string(), c.get(2).unwrap().as_str().parse().unwrap()))
                }
            }
        })
        .collect();

    (coords, folds)
}

#[aoc(day13, part1)]
fn part1((coords, folds): &(HashSet<(i32, i32)>, Vec<(String, i32)>)) -> usize {
    let mut coords = coords.clone();

    if let Some((axis, fold_position)) = folds.first() {
        coords = coords.into_iter()
            .map(|(x, y)| match axis.as_str() {
                "x" => (x, y),
                "y" => (y, x),
                _ => panic!(),
            })
            .map(|(mut x, y)| {
                x = x - fold_position;
                x = x.abs();
                x = fold_position - x;

                (x, y)
            })
            .map(|(x, y)| match axis.as_str() {
                "x" => (x, y),
                "y" => (y, x),
                _ => panic!(),
            })
            .collect();
    }

    coords.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(17, part1(&parse(include_str!("../input/2021/day13.part1.test.17.txt"))));
    }
}