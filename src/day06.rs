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

fn simulate(nums: &[i32], iterations: usize) -> usize {
    let mut counts: [usize; 9] = [0; 9];

    for num in nums {
        counts[*num as usize] += 1
    }

    for i in 0..iterations {
        counts[(i + 1 + 6) % 9] += counts[i % 9];
    }

    counts.iter().sum()
}

#[aoc(day6, part1)]
fn part1(nums: &[i32]) -> usize {
    simulate(nums, 80)
}

#[aoc(day6, part2)]
fn part2(nums: &[i32]) -> usize {
    simulate(nums, 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(5934, part1(&parse(include_str!("../input/2021/day6.part1.test.5934.txt")).unwrap()));
    }

    #[test]
    fn part2_example() {
        assert_eq!(26984457539, part2(&parse(include_str!("../input/2021/day6.part2.test.26984457539.txt")).unwrap()));
    }
}