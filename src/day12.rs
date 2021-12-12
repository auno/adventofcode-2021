use std::collections::{HashMap, VecDeque};
use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;
use itertools::Itertools;

#[aoc_generator(day12)]
fn parse(input: &str) -> Result<HashMap<String, Vec<String>>, ParseIntError> {
    Ok(
        input
            .lines()
            .flat_map(|line| {
                let mut parts = line.split("-");
                let a = parts.next().unwrap().to_string();
                let b = parts.next().unwrap().to_string();

                [(a.clone(), b.clone()), (b, a)]
            })
            .fold(HashMap::new(), |mut acc, (a, b)| {
                acc.entry(a).or_insert(vec![]).push(b);
                acc
            })
    )
}

fn num_paths(map: &&HashMap<String, Vec<String>>, is_eligible: fn(&Vec<&str>, &str) -> bool) -> usize {
    let mut num_paths = 0;
    let mut queue: VecDeque<Vec<&str>> = VecDeque::new();
    queue.push_back(vec!["start"]);

    while let Some(path) = queue.pop_front() {
        let current = path.last().unwrap();

        if current == &"end" {
            num_paths += 1;
            continue;
        }

        for neighbor in &map[&current.to_string()] {
            if is_eligible(&path, neighbor) {
                let mut next_path = path.clone();
                next_path.push(neighbor);
                queue.push_back(next_path);
            }
        }
    }

    num_paths
}

#[aoc(day12, part1)]
fn part1(map: &HashMap<String, Vec<String>>) -> usize {
    num_paths(&map, |path, node| &node.to_uppercase() == node || !path.contains(&node))
}

#[aoc(day12, part2)]
fn part2(map: &HashMap<String, Vec<String>>) -> usize {
    num_paths(&map, |path, node| {
        if node == "start" {
            return false;
        }

        if &node.to_uppercase() == node || !path.contains(&node) {
            return true;
        }

        path
            .iter()
            .filter(|n| n.to_lowercase() == **n)
            .duplicates()
            .count() == 0
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(10, part1(&parse(include_str!("../input/2021/day12.part1.test.10.txt")).unwrap()));
    }

    #[test]
    fn part1_example2() {
        assert_eq!(19, part1(&parse(include_str!("../input/2021/day12.part1.test.19.txt")).unwrap()));
    }

    #[test]
    fn part1_example3() {
        assert_eq!(226, part1(&parse(include_str!("../input/2021/day12.part1.test.226.txt")).unwrap()));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(36, part2(&parse(include_str!("../input/2021/day12.part2.test.36.txt")).unwrap()));
    }

    #[test]
    fn part2_example2() {
        assert_eq!(103, part2(&parse(include_str!("../input/2021/day12.part2.test.103.txt")).unwrap()));
    }

    #[test]
    fn part2_example3() {
        assert_eq!(3509, part2(&parse(include_str!("../input/2021/day12.part2.test.3509.txt")).unwrap()));
    }
}