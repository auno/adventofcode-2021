use std::cmp::max;
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

fn fold(coords: &HashSet<(i32, i32)>, fold_axis: &String, fold_position: &i32) -> HashSet<(i32, i32)> {
    coords.into_iter()
        .map(|(x, y)| match fold_axis.as_str() {
            "x" => (*x, *y),
            "y" => (*y, *x),
            _ => panic!(),
        })
        .map(|(mut x, y)| {
            x = x - fold_position;
            x = x.abs();
            x = fold_position - x;

            (x, y)
        })
        .map(|(x, y)| match fold_axis.as_str() {
            "x" => (x, y),
            "y" => (y, x),
            _ => panic!(),
        })
        .collect()
}

fn print_coords(coords: &HashSet<(i32, i32)>) -> String {
    let mut chars = vec![];

    let (height, width) = coords.into_iter()
        .fold((0, 0), |(h, w), (x, y)| (
            max(h, y + 1),
            max(w, x + 1),
        ));

    for y in 0..height {
        for x in 0..width {
            chars.push(match coords.contains(&(x, y)) {
                true => '▓',
                false => ' ',
            });
        }

        chars.push('\n');
    }

    String::from_iter(chars)
}

#[aoc(day13, part1)]
fn part1((coords, folds): &(HashSet<(i32, i32)>, Vec<(String, i32)>)) -> usize {
    let mut coords = coords.clone();

    if let Some((axis, fold_position)) = folds.first() {
        coords = fold(&coords, axis, fold_position);
    }

    coords.len()
}

#[aoc(day13, part2)]
fn part2((coords, folds): &(HashSet<(i32, i32)>, Vec<(String, i32)>)) -> String {
    let mut coords = coords.clone();

    for (axis, fold_position) in folds {
        coords = fold(&coords, axis, fold_position);
    }

    format!("\n\n{}", print_coords(&coords))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(17, part1(&parse(include_str!("../input/2021/day13.part1.test.17.txt"))));
    }

    #[test]
    fn part2_example() {
        assert_eq!(16, part2(&parse(include_str!("../input/2021/day13.part2.test.16.txt"))).chars().filter(|c| *c == '▓').count());
    }
}