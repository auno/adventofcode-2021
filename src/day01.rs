use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day1)]
fn parse(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input
        .lines()
        .map(|line| line.parse())
        .collect()
}

#[aoc(day1, part1)]
fn part1(nums: &[i32]) -> usize {
    nums
        .windows(2)
        .filter(|w| w[1] > w[0])
        .count()
}

#[aoc(day1, part2)]
fn part2(nums: &[i32]) -> usize {
    let sums: Vec<i32> = nums
        .windows(3)
        .map(|w| w[0] + w[1] + w[2])
        .collect();

    sums
        .windows(2)
        .filter(|w| w[1] > w[0])
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(7, part1(&parse(include_str!("../input/2021/day1.part1.test.7.txt")).unwrap()));
    }

    #[test]
    fn part2_example() {
        assert_eq!(5, part2(&parse(include_str!("../input/2021/day1.part2.test.5.txt")).unwrap()));
    }
}