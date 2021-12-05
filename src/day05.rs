use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[derive(Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Result<Vec<(Point, Point)>, ParseIntError> {
    Ok(
        input
            .lines()
            .map(|line| {
                let coords: Vec<i32> = line
                    .split(" -> ")
                    .flat_map(|part| part.split(","))
                    .flat_map(|coord| coord.parse())
                    .fuse()
                    .collect();

                (Point{ x: coords[0], y: coords[1]}, Point{ x: coords[2], y: coords[3]})
            })
            .collect()
    )
}

fn range(a: i32, b: i32) -> Vec<i32> {
    if b >= a {
        (a..=b).collect::<Vec<i32>>()
    } else {
        (b..=a).rev().collect::<Vec<i32>>()
    }
}

fn mark(points: &mut HashMap<Point, usize>, a: &Point, b: &Point) {
    match (a, b) {
        (c, d) if (c.x == d.x) => {
            for y in range(c.y, d.y) {
                *points.entry(Point { x: c.x, y }).or_default() += 1;
            }
        },
        (c, d) if (c.y == d.y) => {
            for x in range(c.x, d.x) {
                *points.entry(Point { x, y: c.y }).or_default() += 1;
            }
        },
        (c, d) => {
            for (x, y) in range(c.x, d.x).into_iter().zip(range(c.y, d.y)) {
                *points.entry(Point { x, y }).or_default() += 1;
            }
        }
    }
}

#[aoc(day5, part1)]
fn part1(lines: &[(Point, Point)]) -> usize {
    let mut points: HashMap<Point, usize> = HashMap::new();

    for (a, b) in lines {
        if a.x != b.x && a.y != b.y {
            continue;
        }

        mark(&mut points, a, b);
    }

    points
        .iter()
        .filter(|(_, c)| **c > 1)
        .count()
}

#[aoc(day5, part2)]
fn part2(lines: &[(Point, Point)]) -> usize {
    let mut points: HashMap<Point, usize> = HashMap::new();

    for (a, b) in lines {
        mark(&mut points, a, b);
    }

    points
        .iter()
        .filter(|(_, c)| **c > 1)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(5, part1(&parse(include_str!("../input/2021/day5.part1.test.5.txt")).unwrap()));
    }

    #[test]
    fn part2_example() {
        assert_eq!(12, part2(&parse(include_str!("../input/2021/day5.part2.test.12.txt")).unwrap()));
    }
}