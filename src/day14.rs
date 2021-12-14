use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use itertools::Itertools;

#[aoc_generator(day14)]
fn parse(input: &str) -> (Vec<char>, HashMap<(char, char), char>) {
    let template = input.lines().next().unwrap().chars().collect();

    let re = Regex::new(r"^(.)(.) -> (.)$").unwrap();
    let rules = input
        .lines()
        .skip(2)
        .map(|line| {
            let caps = re.captures(line).unwrap();
            (
                (
                    caps.get(1).unwrap().as_str().chars().next().unwrap(),
                    caps.get(2).unwrap().as_str().chars().next().unwrap(),
                ),
                caps.get(3).unwrap().as_str().chars().next().unwrap()
            )
        })
        .collect();

    (template, rules)
}

#[aoc(day14, part1)]
fn part1((template, rules): &(Vec<char>, HashMap<(char, char), char>)) -> usize {
    let mut polymer = template.clone();

    for _step in 0..10 {
        let between: Vec<char> = polymer
            .windows(2)
            .map(|w| rules.get(&(w[0], w[1])).unwrap())
            .cloned()
            .collect();

        polymer = polymer.into_iter().interleave(between.into_iter()).collect();
    }

    let counts = polymer.iter().counts();
    let ((_, min), (_, max)) = counts.iter().minmax_by_key(|(_, count)| **count).into_option().unwrap();

    max - min
}

#[aoc(day14, part2)]
fn part2((template, rules): &(Vec<char>, HashMap<(char, char), char>)) -> usize {
    let mut pairs = template
        .windows(2)
        .map(|w| (w[0], w[1]))
        .counts();

    for _step in 0..40 {
        let mut next_pairs = pairs.clone();

        for ((a, b), count) in pairs.into_iter() {
            if count == 0 {
                continue;
            }

            if let Some(rule) = rules.get(&(a, b)) {
                *next_pairs.entry((a, b)).or_insert(0) -= count;
                *next_pairs.entry((a, *rule)).or_insert(0) += count;
                *next_pairs.entry((*rule, b)).or_insert(0) += count;
            }
        }

        pairs = next_pairs;
    }

    let mut individual_counts: HashMap<char, usize> = pairs
        .iter()
        .flat_map(|((a, b), count)| [(a, count), (b, count)])
        .fold(HashMap::new(), |mut acc, (c, count)| {
            *acc.entry(*c).or_insert(0) += count;
            acc
        });

    *individual_counts.entry(*template.first().unwrap()).or_default() += 1;
    *individual_counts.entry(*template.last().unwrap()).or_default() += 1;

    for (_, count) in individual_counts.iter_mut() {
        *count /= 2;
    }

    let ((_, min), (_, max)) = individual_counts.iter().minmax_by_key(|(_, count)| **count).into_option().unwrap();

    max - min
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(1588, part1(&parse(include_str!("../input/2021/day14.part1.test.1588.txt"))));
    }

    #[test]
    fn part2_example() {
        assert_eq!(2188189693529, part2(&parse(include_str!("../input/2021/day14.part2.test.2188189693529.txt"))));
    }
}