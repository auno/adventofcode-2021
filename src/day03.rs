use std::convert::Infallible;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn parse(input: &str) -> Result<Vec<String>, Infallible> {
    input
        .lines()
        .map(|line| line.parse())
        .collect()
}

#[aoc(day3, part1)]
fn part1(nums: &[String]) -> u32 {
    let mut num_ones = vec![0; nums[0].len()];
    nums.iter()
        .for_each(|num| {
            num.chars()
                .enumerate()
                .for_each(|(i, c)| {
                    if c == '1' {
                        num_ones[i] += 1;
                    }
                })
        });

    let gamma: u32 = num_ones.iter()
        .map(|c| if *c > (nums.len() as i32) - c { 1 } else { 0 })
        .reduce(|acc, b| (acc << 1) + b)
        .unwrap();

    let epsilon: u32 = num_ones.iter()
        .map(|c| if *c < (nums.len() as i32) - c { 1 } else { 0 })
        .reduce(|acc, b| (acc << 1) + b)
        .unwrap();

    gamma * epsilon
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(198, part1(&parse(include_str!("../input/2021/day3.part1.test.198.txt")).unwrap()));
    }
}