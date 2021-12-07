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

fn solve(nums: &[i32], cost: fn (&[i32], i32) -> i32) -> i32 {
    let min = nums.iter().min().unwrap();
    let max = nums.iter().max().unwrap();

    let optimal: i32 = (*min..=*max)
        .map(|candidate| cost(nums, candidate))
        .into_iter()
        .min()
        .unwrap();

    optimal
}

#[aoc(day7, part1)]
fn part1(nums: &[i32]) -> i32 {
    solve(nums, |nums, candidate| {
        nums
            .iter()
            .map(|crab_position| (crab_position - candidate).abs())
            .sum()
    })
}

#[aoc(day7, part2)]
fn part2(nums: &[i32]) -> i32 {
    solve(nums, |crab_positions, candidate| {
        crab_positions
            .iter()
            .map(|crab_position| {
                let distance = (crab_position - candidate).abs();
                (distance.pow(2) + distance) / 2
            })
            .sum()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(37, part1(&parse(include_str!("../input/2021/day7.part1.test.37.txt")).unwrap()));
    }

    #[test]
    fn part2_example() {
        assert_eq!(168, part2(&parse(include_str!("../input/2021/day7.part2.test.168.txt")).unwrap()));
    }
}