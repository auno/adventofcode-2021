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

fn mark(points: &mut HashMap<Point, usize>, a: &Point, b: &Point) {
    match (a, b) {
        (c, d) if (c.x == d.x && c.y <= d.y) => {
            for y in c.y..=d.y {
                *points.entry(Point { x: c.x, y }).or_default() += 1;
            }
        },
        (c, d) if (c.y == d.y && c.x <= d.x) => {
            for x in c.x..=d.x {
                *points.entry(Point { x, y: c.y }).or_default() += 1;
            }
        },
        (c, d) => mark(points, d, c)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(5, part1(&parse(include_str!("../input/2021/day5.part1.test.5.txt")).unwrap()));
    }
}