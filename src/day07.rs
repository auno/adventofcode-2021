use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day7)]
fn parse(input: &str) -> Result<Vec<i32>, ParseIntError> {
    Ok(
        input
            .lines()
            .flat_map(|line| line.split(","))
            .flat_map(|num| num.parse())
            .collect()
    )
}

#[aoc(day7, part1)]
fn part1(nums: &[i32]) -> i32 {
    let min = nums.iter().min().unwrap();
    let max = nums.iter().max().unwrap();

    let optimal: i32 = (*min..=*max)
        .map(|candidate| {
            nums
                .iter()
                .map(|crab_position| (crab_position - candidate).abs())
                .sum()
        })
        .into_iter()
        .min()
        .unwrap();

    optimal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(37, part1(&parse(include_str!("../input/2021/day7.part1.test.37.txt")).unwrap()));
    }
}