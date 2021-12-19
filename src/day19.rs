use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nalgebra::{ArrayStorage, Const, Matrix, Matrix1xX, Matrix3, Matrix3xX, Matrix6x3, MatrixXx1};

fn parse_scanner(input: &str) -> Matrix3xX<i32> {
    Matrix3xX::from_iterator(
        input.lines().filter(|line| !line.starts_with("---")).count(),
        input
            .lines()
            .filter(|line| !line.starts_with("---"))
            .flat_map(|line| line.split(","))
            .map(|num| num.parse::<i32>().unwrap())
    )
}

#[aoc_generator(day19)]
fn parse(input: &str) -> Vec<Matrix3xX<i32>> {
    input
        .split("\n\n")
        .map(|scanner| parse_scanner(scanner))
        .collect()
}

fn rotations() -> [Matrix<i32, Const<3>, Const<3>, ArrayStorage<i32, 3, 3>>; 24] {
    let roti = Matrix3::new(
        1,  0,  0,
        0,  1,  0,
        0,  0,  1,
    );

    let rotx90 = Matrix3::new(
        1,  0,  0,
        0,  0, -1,
        0,  1,  0,
    );
    let rotx180 = rotx90 * rotx90;
    let rotx270 = rotx180 * rotx90;

    let roty90 = Matrix3::new(
        0,  0,  1,
        0,  1,  0,
        -1,  0,  0,
    );
    let roty180 = roty90 * roty90;
    let roty270 = roty180 * roty90;

    let rotz90 = Matrix3::new(
        0, -1,  0,
        1,  0,  0,
        0,  0,  1,
    );
    let rotz180 = rotz90 * rotz90;
    let rotz270 = rotz180 * rotz90;

    [
        roti,           rotx90,           rotx180,           rotx270,
        roti * roty90,  rotx90 * roty90,  rotx180 * roty90,  rotx270 * roty90,
        roti * roty270, rotx90 * roty270, rotx180 * roty270, rotx270 * roty270,
        roti * rotz90,  rotx90 * rotz90,  rotx180 * rotz90,  rotx270 * rotz90,
        roti * rotz180, rotx90 * rotz180, rotx180 * rotz180, rotx270 * rotz180,
        roti * rotz270, rotx90 * rotz270, rotx180 * rotz270, rotx270 * rotz270,
    ]
}

fn find_overlap(known: &Matrix3xX<i32>, candidate: &Matrix3xX<i32>) -> Option<Matrix3xX<i32>> {
    for rotation in rotations() {
        let rotated = rotation * candidate.clone();
        for (ai, bi) in (0..known.ncols()).cartesian_product(0..candidate.ncols()) {
            let ap = known.column(ai);

            let bp = rotated.column(bi);
            let diff = bp - ap;

            debug_assert_eq!(bp - diff, ap);

            let diff = diff * Matrix1xX::repeat(candidate.ncols(), 1);
            let b_adjusted = &rotated - &diff;

            let mut cols = known.column_iter().collect_vec();
            cols.append(&mut b_adjusted.column_iter().collect_vec());

            if known.column_iter().interleave(b_adjusted.column_iter()).counts().iter().filter(|(_, count)| **count > 1).count() >= 12 {
                return Some(rotated - diff);
            }
        }
    }

    None
}

#[aoc(day19, part1)]
fn part1(scanners: &Vec<Matrix3xX<i32>>) -> usize {
    let mut known: Vec<usize> = vec![0];
    let mut resolved: Vec<Matrix3xX<i32>> = vec![scanners[0].clone()];

    while known.len() < scanners.len() {
        'outer: for candidate_index in 0..scanners.len() {
            if known.contains(&candidate_index) {
                continue;
            }

            for reference_index in 0..resolved.len() {
                let reference = &resolved[reference_index];
                let candidate = &scanners[candidate_index];

                if let Some(scanner) = find_overlap(reference, candidate) {
                    known.push(candidate_index);
                    resolved.push(scanner);
                    break 'outer;
                }
            }
        }
    }

    resolved
        .iter()
        .flat_map(|scanner| scanner.column_iter().map(|col| (
            col.as_slice()[0],
            col.as_slice()[1],
            col.as_slice()[2],
        )))
        .unique()
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(79, part1(&parse(include_str!("../input/2021/day19.part1.test.79.txt"))));
    }
}