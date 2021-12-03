use std::convert::Infallible;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn parse(input: &str) -> Result<Vec<Vec<u32>>, Infallible> {
    Ok(
        input
            .lines()
            .map(|line| line.chars().map(|c| c as u32 - 48).collect())
            .collect()
    )
}

fn count_ones(nums: &Vec<Vec<u32>>) -> Vec<usize> {
    nums.iter().cloned()
        .reduce(|acc, num| acc.iter()
            .zip(num.iter())
            .map(|(a, b)| a + b)
            .collect())
        .map(|v| v.iter().map(|c| *c as usize).collect())
        .unwrap()
}

#[aoc(day3, part1)]
fn part1(nums: &Vec<Vec<u32>>) -> u32 {
    let num_ones = count_ones(nums);

    let gamma: u32 = num_ones.iter()
        .map(|c| if *c > nums.len() - c { 1 } else { 0 })
        .reduce(|acc, b| (acc << 1) + b)
        .unwrap();

    let epsilon: u32 = num_ones.iter()
        .map(|c| if *c < nums.len() - c { 1 } else { 0 })
        .reduce(|acc, b| (acc << 1) + b)
        .unwrap();

    gamma * epsilon
}

fn find_rating<T>(nums: &Vec<Vec<u32>>, i: usize, criterion_selector: T) -> Vec<u32> where T: Fn(usize, usize) -> u32 {
    if nums.len() == 1 {
        nums[0].clone()
    } else {
        let num_ones = count_ones(nums);
        let criterion = criterion_selector(nums.len() - num_ones[i], num_ones[i]);
        let filtered_nums: Vec<Vec<u32>> = nums.iter()
            .filter(|num| num[i] == criterion)
            .cloned()
            .collect();

        find_rating(&filtered_nums, i + 1, criterion_selector)
    }
}

#[aoc(day3, part2)]
fn part2(nums: &Vec<Vec<u32>>) -> u32 {
    let rating1 = find_rating(nums, 0, |zeros, ones| if ones >= zeros { 1 } else { 0 })
        .iter()
        .cloned()
        .reduce(|acc, b| (acc << 1) + b)
        .unwrap();
    let rating2 = find_rating(nums, 0, |zeros, ones| if zeros <= ones { 0 } else { 1 })
        .iter()
        .cloned()
        .reduce(|acc, b| (acc << 1) + b)
        .unwrap();

    rating1 * rating2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let parsed_input = parse(include_str!("../input/2021/day3.part1.test.198.txt")).unwrap();
        assert_eq!(198, part1(&parsed_input));
    }

    #[test]
    fn part2_example() {
        let parsed_input = parse(include_str!("../input/2021/day3.part2.test.230.txt")).unwrap();
        assert_eq!(230, part2(&parsed_input));
    }
}