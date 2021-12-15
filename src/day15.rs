use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day15)]
fn parse(input: &str) -> (HashMap<(i32, i32), i32>, i32) {
    let size = input.lines().count();

    (
        input
            .lines()
            .flat_map(|line| line.chars())
            .map(|c| c.to_digit(10).unwrap() as i32)
            .enumerate()
            .map(|(i, v)| (((i / size) as i32, (i % size) as i32), v))
            .collect(),
        size as i32
    )
}

#[aoc(day15, part1)]
fn part1((map, size): &(HashMap<(i32, i32), i32>, i32)) -> i32 {
    let mut cumulative_risk: HashMap<(i32, i32), i32> = HashMap::new();
    let mut queue: BinaryHeap<(Reverse<i32>, i32, i32)> = BinaryHeap::new();

    cumulative_risk.insert((0, 0), 0);
    queue.push((Reverse(0), 0, 0));

    while let Some((Reverse(current_cumulative_risk), current_i, current_j)) = queue.pop() {
        for (neighbor_i, neighbor_j) in [(current_i - 1, current_j), (current_i + 1, current_j), (current_i, current_j - 1), (current_i, current_j + 1)] {
            if let Some(neighbor_risk) = map.get(&(neighbor_i, neighbor_j)) {
                let neighbor_cumulative_risk = cumulative_risk.entry((neighbor_i, neighbor_j)).or_insert(i32::MAX);

                if *neighbor_cumulative_risk > current_cumulative_risk + neighbor_risk {
                    *neighbor_cumulative_risk = current_cumulative_risk + neighbor_risk;
                    queue.push((Reverse(*neighbor_cumulative_risk), neighbor_i, neighbor_j));
                }
            }
        }
    }

    cumulative_risk[&(size - 1, size - 1)]
}


#[aoc(day15, part2)]
fn part2((map, size): &(HashMap<(i32, i32), i32>, i32)) -> i32 {
    let map: HashMap<(i32, i32), i32> = map.iter()
        .flat_map(|((i, j), risk)| {
            [
                ((*i+size*0, *j+size*0), ((risk-1)%9)+1), ((*i+size*1, *j+size*0), ((risk+0)%9)+1), ((*i+size*2, *j+size*0), ((risk+1)%9)+1), ((*i+size*3, *j+size*0), ((risk+2)%9)+1), ((*i+size*4, *j+size*0), ((risk+3)%9)+1),
                ((*i+size*0, *j+size*1), ((risk+0)%9)+1), ((*i+size*1, *j+size*1), ((risk+1)%9)+1), ((*i+size*2, *j+size*1), ((risk+2)%9)+1), ((*i+size*3, *j+size*1), ((risk+3)%9)+1), ((*i+size*4, *j+size*1), ((risk+4)%9)+1),
                ((*i+size*0, *j+size*2), ((risk+1)%9)+1), ((*i+size*1, *j+size*2), ((risk+2)%9)+1), ((*i+size*2, *j+size*2), ((risk+3)%9)+1), ((*i+size*3, *j+size*2), ((risk+4)%9)+1), ((*i+size*4, *j+size*2), ((risk+5)%9)+1),
                ((*i+size*0, *j+size*3), ((risk+2)%9)+1), ((*i+size*1, *j+size*3), ((risk+3)%9)+1), ((*i+size*2, *j+size*3), ((risk+4)%9)+1), ((*i+size*3, *j+size*3), ((risk+5)%9)+1), ((*i+size*4, *j+size*3), ((risk+6)%9)+1),
                ((*i+size*0, *j+size*4), ((risk+3)%9)+1), ((*i+size*1, *j+size*4), ((risk+4)%9)+1), ((*i+size*2, *j+size*4), ((risk+5)%9)+1), ((*i+size*3, *j+size*4), ((risk+6)%9)+1), ((*i+size*4, *j+size*4), ((risk+7)%9)+1),
            ]
        })
        .collect();

    part1(&(map, size * 5))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(40, part1(&parse(include_str!("../input/2021/day15.part1.test.40.txt"))));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(315, part2(&parse(include_str!("../input/2021/day15.part2.test.315.txt"))));
    }

    #[test]
    fn part2_example2() {
        assert_eq!(10, part1(&parse(include_str!("../input/2021/day15.part2.test.10.txt"))));
    }
}