use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};
use crate::day25::Direction::{East, South};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    East,
    South,
}

impl Direction {
    fn neighbor(&self, (i, j): (usize, usize), (height, width): (usize, usize)) -> (usize, usize) {
        match self {
            East => (i, (j + 1) % width),
            South => ((i + 1) % height, j),
        }
    }
}

#[aoc_generator(day25)]
fn parse(input: &str) -> (HashMap<(usize, usize), Direction>, (usize, usize)) {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let map = input
        .lines()
        .flat_map(|line| line.chars())
        .enumerate()
        .filter_map(|(i, c)| match c {
            '>' => Some(((i / width, i % width), East)),
            'v' => Some(((i / width, i % width), South)),
            '.' => None,
            _ => panic!(),
        })
        .collect();

    (map, (height, width))
}

#[aoc(day25, part1)]
fn part1((map, dimensions): &(HashMap<(usize, usize), Direction>, (usize, usize))) -> usize {
    let mut map = map.clone();
    let mut steps = 0;

    loop {
        let mut moved = false;

        for current_direction in [ East, South ] {
            let mut next_map = HashMap::new();
            for (&position, &direction) in &map {
                let neighbor = direction.neighbor(position, *dimensions);

                if direction != current_direction || map.contains_key(&neighbor) || next_map.contains_key(&neighbor) {
                    next_map.insert(position, direction);
                } else {
                    next_map.insert(neighbor, direction);
                    moved = true;
                }
            }
            map = next_map;
        }

        steps += 1;

        if !moved {
            return steps;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(58, part1(&parse(include_str!("../input/2021/day25.part1.test.58.txt"))));
    }
}