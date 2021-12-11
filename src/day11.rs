use std::collections::{HashMap, HashSet};
use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

fn parse_input(input: &str) -> Result<HashMap<(i32, i32), i32>, ParseIntError> {
    let width = input.lines().next().unwrap().chars().count();

    Ok(
        input
            .lines()
            .flat_map(|line| line.chars())
            .map(|c| c.to_digit(10).unwrap() as i32)
            .enumerate()
            .map(|(i, v)| (((i / width) as i32, (i % width) as i32), v))
            .collect()
    )
}

#[aoc_generator(day11)]
fn generate(input: &str) -> Result<(HashMap<(i32, i32), i32>, usize), ParseIntError> {
    Ok((parse_input(input)?, 100))
}

fn neighbors((i, j): &(i32, i32)) -> impl IntoIterator<Item = (i32, i32)> {
    let (i, j) = (*i, *j);

    (0..9)
        .map(|a| (a / 3 - 1, a % 3 - 1))
        .map(move |(a, b)| (i + a, j + b))
        .filter(move |(a, b)| !(*a == i && *b == j))
}

#[aoc(day11, part1)]
fn part1((energy, steps): &(HashMap<(i32, i32), i32>, usize)) -> usize {
    let mut energy = energy.clone();
    let mut num_flashes = 0;

    for _step in 0..*steps {
        for (_, v) in &mut energy {
            *v += 1;
        }

        let mut positions_flashed: HashSet<(i32, i32)> = HashSet::new();

        loop {
            let mut positions_to_increment = vec![];

            for (k, v) in &energy {
                if *v > 9 && !positions_flashed.contains(k) {
                    for neighbor in neighbors(k) {
                        positions_to_increment.push(neighbor);
                    }

                    positions_flashed.insert(k.to_owned());
                }
            }

            if positions_to_increment.is_empty() {
                break;
            }

            for k in positions_to_increment {
                if let Some(v) = energy.get_mut(&k) {
                    *v += 1;
                }
            }
        }

        for (k, v) in &mut energy {
            if positions_flashed.contains(k) {
                *v = 0;
            }
        }

        num_flashes += positions_flashed.len();
    }

    num_flashes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1_10_steps() {
        assert_eq!(204, part1(&(parse_input(include_str!("../input/2021/day11.part1.test.1656.txt")).unwrap(), 10)));
    }

    #[test]
    fn part1_example1_100_steps() {
        assert_eq!(1656, part1(&(parse_input(include_str!("../input/2021/day11.part1.test.1656.txt")).unwrap(), 100)));
    }

    #[test]
    fn part1_example2_2_steps() {
        assert_eq!(9, part1(&(parse_input(include_str!("../input/2021/day11.part1.test.9.txt")).unwrap(), 2)));
    }
}