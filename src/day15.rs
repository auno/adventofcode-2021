use std::collections::{HashMap, VecDeque};
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day15)]
fn parse(input: &str) -> (HashMap<(i32, i32), i32>, usize) {
    let size = input.lines().count();

    (
        input
            .lines()
            .flat_map(|line| line.chars())
            .map(|c| c.to_digit(10).unwrap() as i32)
            .enumerate()
            .map(|(i, v)| (((i / size) as i32, (i % size) as i32), v))
            .collect(),
        size
    )
}

#[aoc(day15, part1)]
fn part1((map, size): &(HashMap<(i32, i32), i32>, usize)) -> i32 {
    let mut distances: HashMap<(i32, i32), i32> = HashMap::new();
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();

    distances.insert((0, 0), 0);
    queue.push_back((0, 0));

    while let Some((ci, cj)) = queue.pop_front() {
        for (ni, nj) in [(ci - 1, cj), (ci + 1, cj), (ci, cj - 1), (ci, cj + 1)] {
            if let Some(risk) = map.get(&(ci, cj)) {
                let cdist = *distances.get(&(ci, cj)).unwrap();
                let ndist = distances.entry((ni, nj)).or_insert(i32::MAX);

                if *ndist > cdist + risk {
                    *ndist = cdist + risk;
                    queue.push_back((ni, nj));
                }
            }
        }
    }

    distances[&((size - 1) as i32, (size - 1) as i32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(40, part1(&parse(include_str!("../input/2021/day15.part1.test.40.txt"))));
    }
}