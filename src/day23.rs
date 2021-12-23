use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
enum Day23Error {
    InvalidAmphipod,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Amphipod {
    A,
    B,
    C,
    D,
}

impl FromStr for Amphipod {
    type Err = Day23Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Amphipod::A),
            "B" => Ok(Amphipod::B),
            "C" => Ok(Amphipod::C),
            "D" => Ok(Amphipod::D),
            _ => Err(Day23Error::InvalidAmphipod)
        }
    }
}

impl Amphipod {
    fn cost(&self) -> i32 {
        match self {
            Amphipod::A => 1,
            Amphipod::B => 10,
            Amphipod::C => 100,
            Amphipod::D => 1000,
        }
    }

    fn target_acceptable(&self, target: usize) -> bool {
        match (self, target) {
            (Amphipod::A, 0|1) => true,
            (Amphipod::B, 2|3) => true,
            (Amphipod::C, 4|5) => true,
            (Amphipod::D, 6|7) => true,
            _ if target > 7 => true,
            _ => false,
        }
    }
}

#[aoc_generator(day23)]
fn parse(input: &str) -> Vec<Amphipod> {
    let re = Regex::new(r"#([ABCD])#([ABCD])#([ABCD])#([ABCD])#").unwrap();
    input
        .lines()
        .filter_map(|line| re.captures(line))
        .flat_map(|caps| (1..5).map(|i| caps.get(i).unwrap().as_str().parse().unwrap()).collect::<Vec<_>>())
        .collect()
}

type State = [Option<Amphipod>; 15];

fn neighbors(moves: &Vec<Vec<(usize, i32, Vec<usize>)>>, state: &State) -> Vec<(State, i32)> {
    state
        .iter()
        .positions(|a| a.is_some())
        .flat_map(|i| moves[i].iter().map(move |(target, distance, between)| (i, *target, *distance, between)))
        /* Filter out occupied targets */
        .filter(|(_, target, _, _)| state[*target].is_none())
        /* Filter out blocked routes */
        .filter(|(_, _, _, between)| between.iter().all(|i| state[*i].is_none()))
        /* Filter out incompatible rooms */
        .filter(|(source, target, _, _)| state[*source].unwrap().target_acceptable(*target))
        /* Filter out upper room if lower is free or if it is occupied by amphipod of other type */
        .filter(|(source, target, _, _)| *target > 7 || *target % 2 == 1 || match (state[*target + 1], state[*source]) {
            (Some(a), Some(b)) if a == b => true,
            _ => false
        })
        /* Filter out hallway targets if already in hallway */
        .filter(|(source, target, _, _)| *target <= 7 || *source <= 7)
        .map(|(source, target, distance, _)| {
            let mut new_state = state.clone();
            new_state[target] = state[source];
            new_state[source] = state[target];

            (new_state, state[source].unwrap().cost() * distance)
        })
        .collect()
}

/**
 * Position mappings (expressed in hexadecimal)
 *
 * #############
 * #89.A.B.C.DE#
 * ###0#2#4#6###
 *   #1#3#5#7#
 *   #########
 */

fn calculate_moves() -> &'static Vec<Vec<(usize, i32, Vec<usize>)>> {
    lazy_static! {
        static ref MOVES_FROM_HOMES: Vec<Vec<(usize, i32, Vec<usize>)>> = (8..15).fold(vec![
            /* From 0 */ vec![(8, 3,  vec![9]),            (9, 2, vec![]),           (10, 2, vec![]),        (11, 4, vec![10]),   (12, 6, vec![10,11]),   (13, 8, vec![10,11,12]),   (14, 9,  vec![10,11,12,13])],
            /* From 1 */ vec![(8, 4,  vec![0,9]),          (9, 3, vec![0]),          (10, 3, vec![0]),       (11, 5, vec![0,10]), (12, 7, vec![0,10,11]), (13, 9, vec![0,10,11,12]), (14, 10, vec![0,10,11,12,13])],
            /* From 2 */ vec![(8, 5,  vec![9,10]),         (9, 4, vec![10]),         (10, 2, vec![]),        (11, 2, vec![]),     (12, 4, vec![11]),      (13, 6, vec![11,12]),      (14, 7,  vec![11,12,13])],
            /* From 3 */ vec![(8, 6,  vec![2,9,10]),       (9, 5, vec![2,10]),       (10, 3, vec![2]),       (11, 3, vec![2]),    (12, 5, vec![2,11]),    (13, 7, vec![2,11,12]),    (14, 8,  vec![2,11,12,13])],
            /* From 4 */ vec![(8, 7,  vec![9,10,11]),      (9, 6, vec![10,11]),      (10, 4, vec![11]),      (11, 2, vec![]),     (12, 2, vec![]),        (13, 4, vec![12]),         (14, 5,  vec![12,13])],
            /* From 5 */ vec![(8, 8,  vec![4,9,10,11]),    (9, 7, vec![4,10,11]),    (10, 5, vec![4,11]),    (11, 3, vec![4]),    (12, 3, vec![4]),       (13, 5, vec![4,12]),       (14, 6,  vec![4,12,13])],
            /* From 6 */ vec![(8, 9,  vec![9,10,11,12]),   (9, 8, vec![10,11,12]),   (10, 6, vec![11,12]),   (11, 4, vec![12]),   (12, 2, vec![]),        (13, 2, vec![]),           (14, 3,  vec![13])],
            /* From 7 */ vec![(8, 10, vec![6,9,10,11,12]), (9, 9, vec![6,10,11,12]), (10, 7, vec![6,11,12]), (11, 5, vec![6,12]), (12, 3, vec![6]),       (13, 3, vec![6]),          (14, 4,  vec![6,13])],
        ], |mut acc, i| {
            acc.push(vec![]);

            for j in 0..8 {
                for k in 0..acc[j].len() {
                    if acc[j][k].0 == i {
                        let (_, distance, between) = acc[j][k].clone();
                        acc[i].push((j, distance, between));
                    }
                }
            }

            acc
        });
    }

    &MOVES_FROM_HOMES as &'static Vec<Vec<(usize, i32, Vec<usize>)>>
}

#[aoc(day23, part1)]
fn part1(rooms: &Vec<Amphipod>) -> i32 {
    let moves = calculate_moves();
    let goal = [
        Some(Amphipod::A), Some(Amphipod::A),
        Some(Amphipod::B), Some(Amphipod::B),
        Some(Amphipod::C), Some(Amphipod::C),
        Some(Amphipod::D), Some(Amphipod::D),
        None, None, None, None, None, None, None,
    ];

    let mut start_state = [None; 15];
    for i in 0..4 {
        start_state[i*2] = Some(rooms[i]);
        start_state[i*2+1] = Some(rooms[4+i]);
    }

    let mut cumulative_cost: HashMap<State, i32> = HashMap::new();
    let mut queue: BinaryHeap<(Reverse<i32>, State)> = BinaryHeap::new();

    cumulative_cost.insert(start_state.clone(), 0);
    queue.push((Reverse(0), start_state.clone()));

    while let Some((Reverse(current_cumulative_cost), state)) = queue.pop() {
        if state == goal {
            break;
        }

        for (neighbor, cost) in neighbors(&moves, &state) {
            let neighbor_cumulative_cost = cumulative_cost.entry(neighbor).or_insert(i32::MAX);

            if *neighbor_cumulative_cost > current_cumulative_cost + cost {
                *neighbor_cumulative_cost = current_cumulative_cost + cost;
                queue.push((Reverse(*neighbor_cumulative_cost), neighbor));
            }
        }
    }

    cumulative_cost[&goal]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbors_test1() {
        let moves = calculate_moves();
        let state = [
            Some(Amphipod::B), Some(Amphipod::A),
            Some(Amphipod::C), Some(Amphipod::D),
            Some(Amphipod::B), Some(Amphipod::C),
            Some(Amphipod::D), Some(Amphipod::A),
            None, None, None, None, None, None, None,
        ];

        let actual = neighbors(&moves, &state);
        assert_eq!(28, actual.len());
    }

    #[test]
    fn part1_example() {
        assert_eq!(12521, part1(&parse(include_str!("../input/2021/day23.part1.test.12521.txt"))));
    }
}