use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
enum Segment {
    A = 0b0000001,
    B = 0b0000010,
    C = 0b0000100,
    D = 0b0001000,
    E = 0b0010000,
    F = 0b0100000,
    G = 0b1000000,
}

#[derive(Debug)]
pub enum SevenSegmentParseError {
    InvalidSegmentFormat(char),
}
impl TryFrom<char> for Segment {
    type Error = SevenSegmentParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'a' => Ok(Segment::A),
            'B' | 'b' => Ok(Segment::B),
            'C' | 'c' => Ok(Segment::C),
            'D' | 'd' => Ok(Segment::D),
            'E' | 'e' => Ok(Segment::E),
            'F' | 'f' => Ok(Segment::F),
            'G' | 'g' => Ok(Segment::G),
            _ => Err(SevenSegmentParseError::InvalidSegmentFormat(value))
        }
    }
}

impl Into<u8> for Segment {
    fn into(self) -> u8 {
        self as u8
    }
}

#[derive(Copy, Clone)]
struct Signal {
    signal: u8,
}

impl FromStr for Signal {
    type Err = SevenSegmentParseError;

    fn from_str(signal: &str) -> Result<Self, Self::Err> {
        signal
            .chars()
            .map(|segment| segment.try_into())
            .fold(Ok(Signal::new()), |acc, segment: Result<Segment, SevenSegmentParseError>| {
                let mut acc = acc?;
                acc.set(segment?);
                Ok(acc)
            })
    }
}

impl Signal {
    fn new() -> Self {
        Self { signal: 0 }
    }

    fn set(&mut self, segment: Segment) {
        let segment: u8 = segment.into();
        self.signal |= segment;
    }

    fn is_set(&self, segment: Segment) -> bool {
        let segment: u8 = segment.into();
        self.signal & segment > 0
    }

    fn num_segments(&self) -> usize {
        self.signal.count_ones() as usize
    }
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Result<Vec<([Signal; 10], [Signal; 4])>, ParseIntError> {
    let mut example_signals: [Signal; 10] = [Signal::new(); 10];
    let mut signals: [Signal; 4] = [Signal::new(); 4];

    Ok(
        input
            .lines()
            .map(|line| line.replace(" | ", " "))
            .map(|line| {
                let all_signals: Vec<Signal> = line.split(" ")
                    .map(|s| s.parse::<Signal>().unwrap())
                    .collect();

                for i in 0..10 {
                    example_signals[i] = all_signals[i];
                }

                for i in 10..14 {
                    signals[i - 10] = all_signals[i];
                }

                (example_signals, signals)
            })
            .collect()
    )
}

#[aoc(day8, part1)]
fn part1(input: &Vec<([Signal; 10], [Signal; 4])>) -> usize {
    input
        .iter()
        .flat_map(|(_, signals)| signals.into_iter())
        .filter(|signal| match signal.num_segments() {
            2 | 3 | 4 | 7 => true,
            _ => false
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(26, part1(&parse(include_str!("../input/2021/day8.part1.test.26.txt")).unwrap()));
    }
}