use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;
use std::ops::{Add, AddAssign, Sub};
use std::slice::Iter;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    InvalidSignalValue(i32),
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

impl Segment {
    fn iter() -> Iter<'static, Segment> {
        static SEGMENTS: [Segment; 7] = [ Segment::A, Segment::B, Segment::C, Segment::D, Segment::E, Segment::F, Segment::G ];
        SEGMENTS.iter()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
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
                acc.set(&segment?);
                Ok(acc)
            })
    }
}

impl TryFrom<i32> for Signal {
    type Error = SevenSegmentParseError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok([ Segment::A, Segment::B, Segment::C, Segment::E, Segment::F, Segment::G ].as_slice().into()),
            1 => Ok([ Segment::C, Segment::F ].as_slice().into()),
            2 => Ok([ Segment::A, Segment::C, Segment::D, Segment::E, Segment::G ].as_slice().into()),
            3 => Ok([ Segment::A, Segment::C, Segment::D, Segment::F, Segment::G ].as_slice().into()),
            4 => Ok([ Segment::B, Segment::C, Segment::D, Segment::F ].as_slice().into()),
            5 => Ok([ Segment::A, Segment::B, Segment::D, Segment::F, Segment::G ].as_slice().into()),
            6 => Ok([ Segment::A, Segment::B, Segment::D, Segment::E, Segment::F, Segment::G ].as_slice().into()),
            7 => Ok([ Segment::A, Segment::C, Segment::F ].as_slice().into()),
            8 => Ok([ Segment::A, Segment::B, Segment::C, Segment::D, Segment::E, Segment::F, Segment::G ].as_slice().into()),
            9 => Ok([ Segment::A, Segment::B, Segment::C, Segment::D, Segment::F, Segment::G ].as_slice().into()),
            _ => Err(SevenSegmentParseError::InvalidSignalValue(value))
        }
    }
}

impl From<&[Segment]> for Signal {
    fn from(segments: &[Segment]) -> Self {
        segments
            .iter()
            .fold(Signal::new(), |mut acc, segment| {
                acc.set(segment);
                acc
            })
    }
}

impl Sub<Signal> for Signal {
    type Output = Self;

    fn sub(self, rhs: Signal) -> Self::Output {
        Signal { signal: self.signal & !rhs.signal }
    }
}

impl Sub<Segment> for Signal {
    type Output = Self;

    fn sub(self, rhs: Segment) -> Self::Output {
        Signal { signal: self.signal & !(rhs as u8) }
    }
}

impl Add<Signal> for Signal {
    type Output = Self;

    fn add(self, rhs: Signal) -> Self::Output {
        Signal { signal: self.signal | rhs.signal }
    }
}

impl Add<Segment> for Signal {
    type Output = Self;

    fn add(self, rhs: Segment) -> Self::Output {
        Signal { signal: self.signal | (rhs as u8) }
    }
}

impl AddAssign<Signal> for Signal {
    fn add_assign(&mut self, rhs: Signal) {
        self.signal |= rhs.signal;
    }
}

impl Signal {
    fn new() -> Self {
        Self { signal: 0 }
    }

    fn set(&mut self, segment: &Segment) {
        let segment: u8 = (*segment).into();
        self.signal |= segment;
    }

    fn is_set(&self, segment: Segment) -> bool {
        let segment: u8 = segment.into();
        self.signal & segment > 0
    }

    fn num_segments(&self) -> usize {
        self.signal.count_ones() as usize
    }

    fn segments(&self) -> impl IntoIterator<Item = Segment> {
        let vec: Vec<Segment> = Segment::iter()
            .filter(|segment| self.is_set(**segment))
            .copied()
            .collect();

        vec
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


#[aoc(day8, part2)]
fn part2(input: &Vec<([Signal; 10], [Signal; 4])>) -> i32 {
    let mut sum = 0;

    for (examples, signals) in input {
        let mut segment_mapping: HashMap<Segment, Segment> = HashMap::new();
        let mut known: HashMap<i32, Signal> = HashMap::new();

        known.insert(1, *examples.iter().find(|e| e.num_segments() == 2).unwrap());
        known.insert(4, *examples.iter().find(|e| e.num_segments() == 4).unwrap());
        known.insert(7, *examples.iter().find(|e| e.num_segments() == 3).unwrap());
        known.insert(8, *examples.iter().find(|e| e.num_segments() == 7).unwrap());
        segment_mapping.insert(Segment::A, (known[&7] - known[&1]).segments().into_iter().next().unwrap());
        known.insert(3, *examples.iter().find(|e| e.num_segments() == 5 && **e + known[&1] == **e).unwrap());
        segment_mapping.insert(Segment::G, (known[&3] - known[&4] - segment_mapping[&Segment::A]).segments().into_iter().next().unwrap());
        segment_mapping.insert(Segment::D, (known[&3] - known[&1] - segment_mapping[&Segment::A] - segment_mapping[&Segment::G]).segments().into_iter().next().unwrap());
        segment_mapping.insert(Segment::B, (known[&4] - known[&1] - segment_mapping[&Segment::D]).segments().into_iter().next().unwrap());
        segment_mapping.insert(Segment::E, (known[&8] - known[&3] - segment_mapping[&Segment::B]).segments().into_iter().next().unwrap());
        known.insert(6, *examples.iter().find(|e| e.num_segments() == 6 && **e + known[&1] != **e).unwrap());
        segment_mapping.insert(Segment::C, (known[&8] - known[&6]).segments().into_iter().next().unwrap());
        segment_mapping.insert(Segment::F, (known[&1] - segment_mapping[&Segment::C]).segments().into_iter().next().unwrap());
        known.insert(0, known[&8] - segment_mapping[&Segment::D]);
        known.insert(2, known[&8] - segment_mapping[&Segment::B] - segment_mapping[&Segment::F]);
        known.insert(5, known[&8] - segment_mapping[&Segment::C] - segment_mapping[&Segment::E]);
        known.insert(7, known[&1] + segment_mapping[&Segment::A]);
        known.insert(9, known[&8] - segment_mapping[&Segment::E]);

        let signal_mapping: HashMap<Signal, i32> = known
            .iter()
            .map(|(k, v)| (*v, *k))
            .collect();

        sum += signals
            .iter()
            .fold(0, |acc, signal| acc * 10 + signal_mapping[signal])
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(26, part1(&parse(include_str!("../input/2021/day8.part1.test.26.txt")).unwrap()));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(61229, part2(&parse(include_str!("../input/2021/day8.part2.test.61229.txt")).unwrap()));
    }

    #[test]
    fn part2_example2() {
        assert_eq!(5353, part2(&parse("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf").unwrap()));
    }
}