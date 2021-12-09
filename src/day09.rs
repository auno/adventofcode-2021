use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day9)]
fn parse(input: &str) -> Result<Vec<Vec<u32>>, ParseIntError> {
    Ok(
        input
            .lines()
            .map(|line| {
                line
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect()
            })
            .collect()
    )
}

#[aoc(day9, part1)]
fn part1(map: &Vec<Vec<u32>>) -> u32 {
    let height = map.len();
    let width = map[0].len();

    let mut sum = 0;

    for i in 0..height {
        for j in 0..width {
            let v = map[i][j];

            if (i > 0 && map[i - 1][j] <= v) || (i < height - 1 && map[i + 1][j] <= v) || (j > 0 && map[i][j - 1] <= v) || (j < width - 1 && map[i][j + 1] <= v) {
                continue;
            }

            sum += v + 1;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(15, part1(&parse(include_str!("../input/2021/day9.part1.test.15.txt")).unwrap()));
    }
}