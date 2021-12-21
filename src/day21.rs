use std::cmp::{max, min};
use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

#[aoc_generator(day21)]
fn parse(input: &str) -> (i32, i32) {
    let nums: Vec<i32> = input
        .lines()
        .map(|line| line.split_once(": ").unwrap().1.parse().unwrap())
        .collect();

    (nums[0], nums[1])
}

fn roll((position, score): &(i32, i32), step: i32) -> (i32, i32) {
    let roll = step * 3;
    let dice = (roll % 100) + 1 + ((roll + 1) % 100) + 1 + ((roll + 2) % 100) + 1;
    let position = ((position + dice - 1) % 10) + 1;
    let score = score + position;

    (position, score)
}

#[aoc(day21, part1)]
fn part1((position_p1, position_p2): &(i32, i32)) -> i32 {
    let (p1, p2, rolls) = (0..)
        .fold_while(((*position_p1, 0), (*position_p2, 0), 0), |(p1, p2, _), step| {
            let (p1, p2) = match step % 2 {
                0 => (roll(&p1, step), p2),
                1 => (p1, roll(&p2, step)),
                _ => unreachable!()
            };

            if p1.1 >= 1000 || p2.1 >= 1000 {
                return Done((p1, p2, (step + 1) * 3));
            }

            Continue((p1, p2, (step + 1) * 3))
        }).into_inner();

    min(p1.1, p2.1) * rolls
}

type Cache = HashMap<((i32, i32), (i32, i32)), (usize, usize)>;

fn simulate(cache: &mut Cache, (position, score): (i32, i32), other_player: (i32, i32)) -> (usize, usize) {
    if other_player.1 >= 21 {
        return (0, 1);
    }

    if let Some(&cached_result) = cache.get(&((position, score), other_player)) {
        return cached_result;
    }

    let result = [ (3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1) ].iter()
        .fold((0, 0), |(current_player_wins, other_player_wins), &(outcome, multiplier)| {
            let position = ((position + outcome - 1) % 10) + 1;
            let wins = simulate(cache, other_player, (position, score + position));
            (current_player_wins + multiplier * wins.1, other_player_wins + multiplier * wins.0)
        });
    cache.insert(((position, score), other_player), result);

    result
}

#[aoc(day21, part2)]
fn part2((position_p1, position_p2): &(i32, i32)) -> usize {
    let mut cache: Cache = HashMap::new();
    let (wins_p1, wins_p2) = simulate(&mut cache, (*position_p1, 0), (*position_p2, 0));
    max(wins_p1, wins_p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(739785, part1(&parse(include_str!("../input/2021/day21.part1.test.739785.txt"))));
    }

    #[test]
    fn part2_example() {
        assert_eq!(444356092776315, part2(&parse(include_str!("../input/2021/day21.part2.test.444356092776315.txt"))));
    }
}