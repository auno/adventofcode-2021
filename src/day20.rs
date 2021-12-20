use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug)]
enum Day20Error {
    InvalidPixel(char),
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Pixel {
    Dark,
    Light,
}

impl Display for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Pixel::Dark => ' ',
            Pixel::Light => '▓',
        })
    }
}

impl TryFrom<char> for Pixel {
    type Error = Day20Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Pixel::Dark),
            '#' => Ok(Pixel::Light),
            _ => Err(Day20Error::InvalidPixel(value)),
        }
    }
}

type Image = HashMap<(i32, i32), Pixel>;

fn parse_image(image: &str) -> Image {
    image
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(j, c)| ((i as i32, j as i32,), c.try_into().unwrap()))
        })
        .collect::<HashMap<_, _>>()
}

#[aoc_generator(day20)]
fn parse(input: &str) -> (Vec<char>, Image) {
    let (algorithm, image) = input.split_once("\n\n").unwrap();
    let algorithm = algorithm.chars().collect::<Vec<_>>();
    let image = parse_image(image);

    (algorithm, image)
}

fn image_bounds(image: &Image) -> (i32, i32, i32, i32) {
    image
        .iter()
        .fold((0, 0, 0, 0), |(acc_min_i, acc_max_i, acc_min_j, acc_max_j), ((i, j), _)| (
            min(acc_min_i, *i),
            max(acc_max_i, *i),
            min(acc_min_j, *j),
            max(acc_max_j, *j),
        ))
}

fn print_image(image: &Image, oob: Pixel) {
    let (min_i, max_i, min_j, max_j) = image_bounds(image);

    for i in min_i..=max_i {
        for j in min_j..=max_j {
            eprint!("{}", image.get(&(i, j)).unwrap_or(&oob));
        }

        eprintln!()
    }
}

fn enhance(algorithm: &Vec<char>, image: &Image, oob: Pixel) -> (Image, Pixel) {
    let mut enhanced_image = HashMap::new();
    let (min_i, max_i, min_j, max_j) = image_bounds(image);


    for (i, j) in ((min_i-1)..=(max_i+1)).cartesian_product((min_j-1)..=(max_j+1)) {
        let index_pixels: [(i32, i32); 9] = [
            (i-1, j-1), (i-1, j), (i-1, j+1),
            (i,   j-1), (i,   j), (i,   j+1),
            (i+1, j-1), (i+1, j), (i+1, j+1),
        ];

        let index = index_pixels
            .iter()
            .map(|bit_index| match image.get(bit_index).unwrap_or(&oob) {
                Pixel::Dark => 0,
                Pixel::Light => 1,
            })
            .reduce(|acc, bit| (acc << 1) + bit)
            .unwrap();

        enhanced_image.insert((i, j), algorithm[index].try_into().unwrap());
    }

    let enhanced_oob: Pixel = match oob {
        Pixel::Dark => algorithm[0].try_into().unwrap(),
        Pixel::Light => algorithm[255].try_into().unwrap(),
    };

    (enhanced_image, enhanced_oob)
}

#[aoc(day20, part1)]
fn part1((algorithm, image): &(Vec<char>, Image)) -> usize {
    let mut image = image.clone();
    let mut oob = Pixel::Dark;

    for _step in 0..2 {
        let (enhanced_image, enhanced_oob) = enhance(algorithm, &image, oob);
        image = enhanced_image;
        oob = enhanced_oob;
    }

    image
        .values()
        .filter(|p| **p == Pixel::Light)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(35, part1(&parse(include_str!("../input/2021/day20.part1.test.35.txt"))));
    }

    #[test]
    fn part1_test0() {
        let (algorithm, _) = parse(include_str!("../input/2021/day20.part1.test.35.txt"));
        let image = parse_image("...\n...\n...\n");
        assert_eq!(0, part1(&(algorithm, image)));
    }
}