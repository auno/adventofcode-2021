use std::collections::HashSet;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nalgebra::{ArrayStorage, Const, Matrix, Matrix1xX, Matrix3, Matrix3x1, Matrix3xX};

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

fn test_overlap(known: &Matrix3xX<i32>, candidate: &Matrix3xX<i32>) -> Option<(Matrix3x1<i32>, Matrix3xX<i32>)> {
    for rotation in rotations() {
        let candidate_rotated = rotation * candidate.clone();

        for (ai, bi) in (0..known.ncols()).cartesian_product(0..candidate.ncols()) {
            let ap = known.column(ai);
            let bp = candidate_rotated.column(bi);
            let candidate_location = ap - bp;
            let normalizer = candidate_location * Matrix1xX::repeat(candidate.ncols(), 1);
            let candidate_normalized = &candidate_rotated + &normalizer;

            if known.column_iter().interleave(candidate_normalized.column_iter()).counts().iter().filter(|(_, count)| **count > 1).count() >= 12 {
                return Some((candidate_location, candidate_normalized));
            }
        }
    }

    None
}

fn find_overlap(scanners: &Vec<Matrix3xX<i32>>) -> Vec<(usize, Matrix3x1<i32>, Matrix3xX<i32>)> {
    let mut resolved: Vec<(usize, Matrix3x1<i32>, Matrix3xX<i32>)> = vec![(0, Matrix3x1::new(0, 0, 0), scanners[0].clone())];
    let mut incompatibles = HashSet::new();

    while resolved.len() < scanners.len() {
        'outer: for candidate_index in 0..scanners.len() {
            if resolved.iter().any(|(reference_index, _, _)| candidate_index == *reference_index) {
                continue;
            }

            for resolved_index in 0..resolved.len() {
                let (reference_index, _, reference) = &resolved[resolved_index];

                if incompatibles.contains(&(*reference_index, candidate_index)) {
                    continue;
                }

                let candidate = &scanners[candidate_index];

                if let Some((location, scanner)) = test_overlap(reference, candidate) {
                    resolved.push((candidate_index, location, scanner));
                    break 'outer;
                } else {
                    incompatibles.insert((*reference_index, candidate_index));
                }
            }
        }
    }

    resolved
}

#[aoc(day19, part1)]
fn part1(scanners: &Vec<Matrix3xX<i32>>) -> usize {
    find_overlap(scanners)
        .iter()
        .flat_map(|(_, _, scanner)| scanner.column_iter())
        .unique()
        .count()
}

#[aoc(day19, part2)]
fn part2(scanners: &Vec<Matrix3xX<i32>>) -> i32 {
    let overlap = find_overlap(scanners);

    (0..overlap.len()).cartesian_product(0..overlap.len())
        .map(|(i, j)| {
            let (_, a, _) = &overlap[i];
            let (_, b, _) = &overlap[j];

            (a-b).abs().sum()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(79, part1(&parse(include_str!("../input/2021/day19.part1.test.79.txt"))));
    }

    #[test]
    fn part2_example() {
        assert_eq!(3621, part2(&parse(include_str!("../input/2021/day19.part2.test.3621.txt"))));
    }
}