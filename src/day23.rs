use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub enum Day23Error {
    InvalidAmphipod,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Amphipod {
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

    fn target_acceptable(&self, target: usize, part2: bool) -> bool {
        match (self, target, part2) {
            (Amphipod::A, 0|1, false) => true,
            (Amphipod::B, 2|3, false) => true,
            (Amphipod::C, 4|5, false) => true,
            (Amphipod::D, 6|7, false) => true,
            (_, _, false) if target >= 8 => true,

            (Amphipod::A, 0|1|2|3, true) => true,
            (Amphipod::B, 4|5|6|7, true) => true,
            (Amphipod::C, 8|9|10|11, true) => true,
            (Amphipod::D, 12|13|14|15, true) => true,
            (_, _, true) if target >= 16 => true,

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

mod part1 {
    use super::*;

    type State = [Option<Amphipod>; 15];

    pub fn neighbors(moves: &Vec<Vec<(usize, i32, Vec<usize>)>>, state: &State) -> Vec<(State, i32)> {
        state
            .iter()
            .positions(|a| a.is_some())
            .flat_map(|i| moves[i].iter().map(move |(target, distance, between)| (i, *target, *distance, between)))
            /* Filter out occupied targets */
            .filter(|(_, target, _, _)| state[*target].is_none())
            /* Filter out blocked routes */
            .filter(|(_, _, _, between)| between.iter().all(|i| state[*i].is_none()))
            /* Filter out incompatible rooms */
            .filter(|(source, target, _, _)| state[*source].unwrap().target_acceptable(*target, false))
            /* Filter out upper room if lower is free or if it is occupied by amphipod of other type */
            .filter(|(source, target, _, _)| *target >= 8 || *target % 2 == 1 || match (state[*target + 1], state[*source]) {
                (Some(a), Some(b)) if a == b => true,
                _ => false
            })
            /* Filter out hallway targets if already in hallway */
            .filter(|(source, target, _, _)| *target < 8 || *source < 8)
            .map(|(source, target, distance, _)| {
                let mut new_state = state.clone();
                new_state[target] = state[source];
                new_state[source] = state[target];

                (new_state, state[source].unwrap().cost() * distance)
            })
            .collect()
    }

    /*
     * Position mappings (expressed in hexadecimal)
     *
     * #############
     * #89.A.B.C.DE#
     * ###0#2#4#6###
     *   #1#3#5#7#
     *   #########
     */

    pub fn calculate_moves() -> &'static Vec<Vec<(usize, i32, Vec<usize>)>> {
        lazy_static! {
        static ref MOVES_FROM_HOMES: Vec<Vec<(usize, i32, Vec<usize>)>> = (0..15).fold(vec![
            /* From 0 */ vec![(8, 3,  vec![9]),          (9, 2, vec![]),         (10, 2, vec![]),      (11, 4, vec![10]), (12, 6, vec![10,11]), (13, 8, vec![10,11,12]), (14, 9,  vec![10,11,12,13])],
                vec![],
            /* From 2 */ vec![(8, 5,  vec![9,10]),       (9, 4, vec![10]),       (10, 2, vec![]),      (11, 2, vec![]),   (12, 4, vec![11]),    (13, 6, vec![11,12]),    (14, 7,  vec![11,12,13])],
                vec![],
            /* From 4 */ vec![(8, 7,  vec![9,10,11]),    (9, 6, vec![10,11]),    (10, 4, vec![11]),    (11, 2, vec![]),   (12, 2, vec![]),      (13, 4, vec![12]),       (14, 5,  vec![12,13])],
                vec![],
            /* From 6 */ vec![(8, 9,  vec![9,10,11,12]), (9, 8, vec![10,11,12]), (10, 6, vec![11,12]), (11, 4, vec![12]), (12, 2, vec![]),      (13, 2, vec![]),         (14, 3,  vec![13])],
                vec![],
        ], |mut acc, i| {
            if i < 8 && i % 2 > 0 {
                for (target, distance, mut between) in acc[i - 1].clone() {
                    between.push(i - 1);
                    acc[i].push((target, distance + 1, between));
                }
            } else if i >= 8 {
                acc.push(vec![]);

                for j in 0..8 {
                    for k in 0..acc[j].len() {
                        if acc[j][k].0 == i {
                            let (_, distance, between) = acc[j][k].clone();
                            acc[i].push((j, distance, between));
                        }
                    }
                }
            }

            acc
        });
    }

        &MOVES_FROM_HOMES as &'static Vec<Vec<(usize, i32, Vec<usize>)>>
    }

    #[aoc(day23, part1)]
    pub fn part1(rooms: &Vec<Amphipod>) -> i32 {
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
            start_state[i * 2] = Some(rooms[i]);
            start_state[i * 2 + 1] = Some(rooms[4 + i]);
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
}

mod part2 {
    use super::*;

    type State = [Option<Amphipod>; 23];

    pub fn neighbors(moves: &Vec<Vec<(usize, i32, Vec<usize>)>>, state: &State) -> Vec<(State, i32)> {
        state
            .iter()
            .positions(|a| a.is_some())
            .flat_map(|i| moves[i].iter().map(move |(target, distance, between)| (i, *target, *distance, between)))
            /* Filter out occupied targets */
            .filter(|(_, target, _, _)| state[*target].is_none())
            /* Filter out blocked routes */
            .filter(|(_, _, _, between)| between.iter().all(|i| state[*i].is_none()))
            /* Filter out incompatible rooms */
            .filter(|(source, target, _, _)| state[*source].unwrap().target_acceptable(*target, true))
            /* Filter out upper rooms if lower is free or if it is occupied by amphipod of other type */
            .filter(|(source, target, _, _)| *target >= 16 || (0..4).all(|i| {
                let j = target - (target % 4) + i;
                (j <= *target && state[j].is_none()) || (j > *target && state[j].is_some() && state[j].unwrap() == state[*source].unwrap())
            }))
            /* Filter out hallway targets if already in hallway */
            .filter(|(source, target, _, _)| *target < 16 || *source < 16)
            .map(|(source, target, distance, _)| {
                let mut new_state = state.clone();
                new_state[target] = state[source];
                new_state[source] = state[target];

                (new_state, state[source].unwrap().cost() * distance)
            })
            .collect()
    }

    /*
     * Position mappings (expressed in hexadecimal)
     *
     *        #############
     * 0x10 + #01.2.3.4.56# aka 16-22
     *        ###0#4#8#C###
     *          #1#5#9#D#
     *          #2#6#A#E#
     *          #3#7#B#F#
     *          #########
     */

    pub fn calculate_moves() -> &'static Vec<Vec<(usize, i32, Vec<usize>)>> {
        lazy_static! {
            static ref MOVES_FROM_HOMES: Vec<Vec<(usize, i32, Vec<usize>)>> = (0..23).fold(vec![
                /* From  0 */ vec![(16, 3,  vec![17]),          (17, 2, vec![]),         (18, 2, vec![]),      (19, 4, vec![18]), (20, 6, vec![18,19]), (21, 8,  vec![18,19,20]), (22, 9,  vec![18,19,20,21])],
                              vec![], vec![], vec![],
                /* From  4 */ vec![(16, 5,  vec![17,18]),       (17, 4, vec![18]),       (18, 2, vec![]),      (19, 2, vec![]),   (20, 4, vec![19]),    (21, 6,  vec![19,20]),    (22, 7,  vec![19,20,21])],
                              vec![], vec![], vec![],
                /* From  8 */ vec![(16, 7,  vec![17,18,19]),    (17, 6, vec![18,19]),    (18, 4, vec![19]),    (19, 2, vec![]),   (20, 2, vec![]),      (21, 4,  vec![20]),       (22, 5,  vec![20,21])],
                              vec![], vec![], vec![],
                /* From 12 */ vec![(16, 9,  vec![17,18,19,20]), (17, 8, vec![18,19,20]), (18, 6, vec![19,20]), (19, 4, vec![20]), (20, 2, vec![]),      (21, 2,  vec![]),         (22, 3,  vec![21])],
                              vec![], vec![], vec![],
            ], |mut acc, i| {
                if i < 16 && i % 4 > 0 {
                    for (target, distance, mut between) in acc[i - 1].clone() {
                        between.push(i - 1);
                        acc[i].push((target, distance + 1, between));
                    }
                } else if i >= 16 {
                    acc.push(vec![]);

                    for j in 0..16 {
                        for k in 0..acc[j].len() {
                            if acc[j][k].0 == i {
                                let (_, distance, between) = acc[j][k].clone();
                                acc[i].push((j, distance, between));
                            }
                        }
                    }
                }

                acc
            });
        }

        &MOVES_FROM_HOMES as &'static Vec<Vec<(usize, i32, Vec<usize>)>>
    }

    #[aoc(day23, part2)]
    pub fn part2(rooms: &Vec<Amphipod>) -> i32 {
        let moves = calculate_moves();
        let goal = [
            Some(Amphipod::A), Some(Amphipod::A), Some(Amphipod::A), Some(Amphipod::A),
            Some(Amphipod::B), Some(Amphipod::B), Some(Amphipod::B), Some(Amphipod::B),
            Some(Amphipod::C), Some(Amphipod::C), Some(Amphipod::C), Some(Amphipod::C),
            Some(Amphipod::D), Some(Amphipod::D), Some(Amphipod::D), Some(Amphipod::D),
            None, None, None, None, None, None, None,
        ];

        let mut start_state = [None; 23];
        for i in 0..4 {
            start_state[i * 4] = Some(rooms[i]);
            start_state[i * 4 + 3] = Some(rooms[4 + i]);
        }

        start_state[ 1] = Some(Amphipod::D);
        start_state[ 2] = Some(Amphipod::D);
        start_state[ 5] = Some(Amphipod::C);
        start_state[ 6] = Some(Amphipod::B);
        start_state[ 9] = Some(Amphipod::B);
        start_state[10] = Some(Amphipod::A);
        start_state[13] = Some(Amphipod::A);
        start_state[14] = Some(Amphipod::C);

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbors_test1() {
        let moves = part1::calculate_moves();
        let state = [
            Some(Amphipod::B), Some(Amphipod::A),
            Some(Amphipod::C), Some(Amphipod::D),
            Some(Amphipod::B), Some(Amphipod::C),
            Some(Amphipod::D), Some(Amphipod::A),
            None, None, None, None, None, None, None,
        ];

        let actual = part1::neighbors(&moves, &state);
        assert_eq!(28, actual.len());
    }

    #[test]
    fn part1_example() {
        assert_eq!(12521, part1::part1(&parse(include_str!("../input/2021/day23.part1.test.12521.txt"))));
    }

    #[test]
    fn part2_example() {
        assert_eq!(44169, part2::part2(&parse(include_str!("../input/2021/day23.part2.test.44169.txt"))));
    }
}