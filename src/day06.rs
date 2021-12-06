use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day6)]
fn parse(input: &str) -> Result<Vec<i32>, ParseIntError> {
    Ok(
        input
            .lines()
            .flat_map(|line| line.split(","))
            .flat_map(|num| num.parse())
            .collect()
    )
}

#[aoc(day6, part1)]
fn part1(nums: &[i32]) -> usize {
    let mut nums: Vec<i32> = nums.iter().copied().collect();
    let mut next_nums: Vec<i32> = vec![];

    for _iteration in 0..80 {
        for num in &nums {
            match num {
                0 => { next_nums.push(6); next_nums.push(8); }
                v => { next_nums.push(v - 1); }
            }
        }

        nums = next_nums;
        next_nums = vec![];
    }

    nums.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(5934, part1(&parse(include_str!("../input/2021/day6.part1.test.5934.txt")).unwrap()));
    }
}