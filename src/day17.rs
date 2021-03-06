use std::cmp::max;
use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;
use itertools::Itertools;
use regex::Regex;

#[aoc_generator(day17)]
fn parse(input: &str) -> Result<(i32, i32, i32, i32), ParseIntError> {
    let re = Regex::new(r"target area: x=(\d+)..(\d+), y=(-?\d+)..(-?\d+)").unwrap();
    let captures = re.captures(input).unwrap();

    Ok((
        captures.get(1).unwrap().as_str().parse()?,
        captures.get(2).unwrap().as_str().parse()?,
        captures.get(3).unwrap().as_str().parse()?,
        captures.get(4).unwrap().as_str().parse()?,
    ))
}

fn is_valid(x_min: &i32, x_max: &i32, y_min: &i32, y_max: &i32, yv: &i32, xv: &i32) -> bool {
    let (mut xv, mut yv) = (*xv, *yv);
    let (mut x, mut y) = (0, 0);

    while x <= *x_max && y >= *y_min {
        x += xv;
        y += yv;
        xv = max(0, xv - 1);
        yv -= 1;

        if (x_min..=x_max).contains(&&x) && (y_min..=y_max).contains(&&y) {
            return true;
        }
    }

    false
}

#[aoc(day17, part1)]
fn part1((x_min, x_max, y_min, y_max): &(i32, i32, i32, i32)) -> i32 {
    let (max_yv, _) = (0..=y_min.abs()).rev().cartesian_product(0..=*x_max)
        .find(|(yv, xv)| is_valid(x_min, x_max, y_min, y_max, yv, xv))
        .unwrap();

    (max_yv*(max_yv+1))/2
}

#[aoc(day17, part2)]
fn part2((x_min, x_max, y_min, y_max): &(i32, i32, i32, i32)) -> usize {
    (-y_min.abs()..=y_min.abs()).rev().cartesian_product(0..=*x_max)
        .filter(|(yv, xv)| is_valid(x_min, x_max, y_min, y_max, yv, xv))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(45, part1(&parse(include_str!("../input/2021/day17.part1.test.45.txt")).unwrap()));
    }

    #[test]
    fn part2_example() {
        assert_eq!(112, part2(&parse(include_str!("../input/2021/day17.part2.test.112.txt")).unwrap()));
    }
}