use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[derive(Debug, Clone)]
struct BingoBoard {
    nums: [i32; 25],
    marked: Vec<i32>,
    completed: bool
}

impl BingoBoard {
    pub fn new(nums: &[i32]) -> Self {
        let mut board = Self {
            nums: [-1; 25],
            marked: vec![],
            completed: false
        };

        nums.iter()
            .enumerate()
            .for_each(|(i, v)| board.nums[i] = *v);

        board
    }

    pub fn mark(&mut self, num: i32) {
        self.marked.push(num);

        if let Some(num_index) = self.nums.iter().position(|n| n == &num) {
            let row = num_index / 5;
            let column = num_index % 5;

            self.completed = self.completed || self.is_row_completed(row) || self.is_column_completed(column);
        }
    }

    pub fn unmarked(&self) -> Vec<i32> {
        self.nums.iter()
            .filter(|num| !self.marked.contains(num))
            .cloned()
            .collect()
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    fn is_row_completed(&self, i: usize) -> bool {
        (0..5).all(|j| self.marked.contains(&self.nums[i * 5 + j]))
    }

    fn is_column_completed(&self, i: usize) -> bool {
        (0..5).all(|j| self.marked.contains(&self.nums[j * 5 + i]))
    }
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Result<(Vec<i32>, Vec<BingoBoard>), ParseIntError> {
    let mut lines = input.lines();
    let nums = lines
        .next()
        .unwrap()
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect();

    let input_nums: Vec<i32> = lines
        .flat_map(|line| line.trim().split_whitespace().map(|num| num.parse().unwrap()))
        .collect();

    let boards: Vec<BingoBoard> = input_nums.chunks(25)
        .map_while(|chunk| {
            match chunk {
                v if v.len() == 0 => None,
                v if v.len() == 25 => Some(BingoBoard::new(&v)),
                _ => panic!("Malformed input")
            }
        })
        .collect();

    Ok((nums, boards))
}

#[aoc(day4, part1)]
fn part1((nums, boards): &(Vec<i32>, Vec<BingoBoard>)) -> i32 {
    let mut boards = boards.clone();
    for num in nums {
        for board in &mut boards {
            board.mark(*num);

            if board.is_completed() {
                let sum_unmarked: i32 = board.unmarked().iter().sum();
                return *num * sum_unmarked;
            }
        }
    }

    panic!("Shouldn't reach here for well-formed input")
}

#[aoc(day4, part2)]
fn part2((nums, boards): &(Vec<i32>, Vec<BingoBoard>)) -> i32 {
    let mut boards = boards.clone();
    for num in nums {
        for board in &mut boards {
            board.mark(*num);
        }

        if boards.len() == 1 && boards[0].is_completed() {
            let sum_unmarked: i32 = boards[0].unmarked().iter().sum();
            return *num * sum_unmarked;
        }

        boards.retain(|b| !b.is_completed());
    }

    panic!("Shouldn't reach here for well-formed input")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(4512, part1(&parse(include_str!("../input/2021/day4.part1.test.4512.txt")).unwrap()));
    }

    #[test]
    fn part2_example() {
        assert_eq!(1924, part2(&parse(include_str!("../input/2021/day4.part2.test.1924.txt")).unwrap()));
    }
}