use std::collections::{HashSet, VecDeque};
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

fn find_low_points(map: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut low_points = vec![];

    let height = map.len();
    let width = map[0].len();

    for i in 0..height {
        for j in 0..width {
            let v = map[i][j];

            if (i > 0 && map[i - 1][j] <= v) || (i < height - 1 && map[i + 1][j] <= v) || (j > 0 && map[i][j - 1] <= v) || (j < width - 1 && map[i][j + 1] <= v) {
                continue;
            }

            low_points.push((i, j));
        }
    }

    low_points
}

fn find_basin_size(map: &Vec<Vec<u32>>, (i, j): (usize, usize)) -> usize {
    let height = map.len();
    let width = map[0].len();

    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    queue.push_back((i, j));
    seen.insert((i, j));

    while let Some((k, l)) = queue.pop_front() {
        if k > 0 && !seen.contains(&(k - 1, l)) && map[k - 1][l] != 9 {
            queue.push_back((k - 1, l));
            seen.insert((k - 1, l));
        }

        if k < height - 1 && !seen.contains(&(k + 1, l)) && map[k + 1][l] != 9 {
            queue.push_back((k + 1, l));
            seen.insert((k + 1, l));
        }

        if l > 0 && !seen.contains(&(k, l - 1)) && map[k][l - 1] != 9 {
            queue.push_back((k, l - 1));
            seen.insert((k, l - 1));
        }

        if l < width - 1 && !seen.contains(&(k, l + 1)) && map[k][l + 1] != 9 {
            queue.push_back((k, l + 1));
            seen.insert((k, l + 1));
        }
    }

    seen.len()
}

#[aoc(day9, part1)]
fn part1(map: &Vec<Vec<u32>>) -> u32 {
    find_low_points(map)
        .into_iter()
        .map(|(i, j)| map[i][j] + 1)
        .sum()
}

#[aoc(day9, part2)]
fn part2(map: &Vec<Vec<u32>>) -> usize {
    let mut basin_sizes: Vec<usize> = find_low_points(map)
        .into_iter()
        .map(|(i, j)| find_basin_size(map, (i, j)))
        .collect();

    basin_sizes.sort();

    basin_sizes
        .iter()
        .rev()
        .take(3)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(15, part1(&parse(include_str!("../input/2021/day9.part1.test.15.txt")).unwrap()));
    }

    #[test]
    fn part2_example() {
        assert_eq!(1134, part2(&parse(include_str!("../input/2021/day9.part2.test.1134.txt")).unwrap()));
    }
}