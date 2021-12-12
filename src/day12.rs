use std::collections::{HashMap, VecDeque};
use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

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

#[aoc(day12, part1)]
fn part1(nums: &HashMap<String, Vec<String>>) -> usize {
    let mut num_paths = 0;
    let mut queue: VecDeque<Vec<&str>> = VecDeque::new();
    queue.push_back(vec!["start"]);

    while let Some(path) = queue.pop_front() {
        let current = path.last().unwrap();

        if current == &"end" {
            num_paths += 1;
            continue;
        }

        for neighbor in &nums[&current.to_string()] {
            if &neighbor.to_uppercase() == neighbor || !path.contains(&neighbor.as_str()) {
                let mut next_path = path.clone();
                next_path.push(neighbor);
                queue.push_back(next_path);
            }
        }
    }

    num_paths
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
}