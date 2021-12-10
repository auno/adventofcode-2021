use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;
use crate::day10::Token::*;

#[derive(Debug)]
pub enum SyntaxError {
    InvalidToken(char),
}

#[derive(Clone, Copy)]
enum Token {
    ParenOpen,
    ParenClose,
    SquareOpen,
    SquareClose,
    CurlyOpen,
    CurlyClose,
    AngleOpen,
    AngleClose,
}

impl TryFrom<char> for Token {
    type Error = SyntaxError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' => Ok(ParenOpen),
            ')' => Ok(ParenClose),
            '[' => Ok(SquareOpen),
            ']' => Ok(SquareClose),
            '{' => Ok(CurlyOpen),
            '}' => Ok(CurlyClose),
            '<' => Ok(AngleOpen),
            '>' => Ok(AngleClose),
            _ => Err(SyntaxError::InvalidToken(value))
        }
    }
}

impl Token {
    fn matches(opening: &Token, closing: &Token) -> bool {
        match (opening, closing) {
            (ParenOpen, ParenClose) => true,
            (SquareOpen, SquareClose) => true,
            (CurlyOpen, CurlyClose) => true,
            (AngleOpen, AngleClose) => true,
            _ => false
        }
    }

    fn score(&self) -> i64 {
        match self {
            ParenOpen => 1,
            SquareOpen => 2,
            CurlyOpen => 3,
            AngleOpen => 4,
            ParenClose => 3,
            SquareClose => 57,
            CurlyClose => 1197,
            AngleClose => 25137,
        }
    }
}

#[aoc_generator(day10)]
fn parse(input: &str) -> Result<Vec<Vec<Token>>, ParseIntError> {
    Ok(
        input
            .lines()
            .map(|line| line
                .chars()
                .map(|c| Token::try_from(c).unwrap())
                .collect::<Vec<Token>>()
            )
            .collect()
    )
}

#[aoc(day10, part1)]
fn part1(lines: &Vec<Vec<Token>>) -> i64 {
    let mut score = 0;

    for line in lines {
        let mut stack: Vec<&Token> = Vec::new();

        for t in line {
            match t {
                ParenOpen | SquareOpen | CurlyOpen | AngleOpen => { stack.push(t); },
                ParenClose | SquareClose | CurlyClose | AngleClose => {
                    match (stack.pop(), t) {
                        (Some(opening), closing) if Token::matches(opening, closing) => {}
                        (Some(_), closing) => {
                            score += closing.score();
                            break;
                        }
                        (None, _) => panic!()
                    }
                }
            }
        }
    }

    score
}

#[aoc(day10, part2)]
fn part2(lines: &Vec<Vec<Token>>) -> i64 {
    let mut scores: Vec<i64> = vec![];

    'outer: for line in lines {
        let mut stack: Vec<&Token> = Vec::new();

        for t in line {
            match t {
                ParenOpen | SquareOpen | CurlyOpen | AngleOpen => { stack.push(t); },
                ParenClose | SquareClose | CurlyClose | AngleClose => {
                    match (stack.pop(), t) {
                        (Some(opening), closing) if Token::matches(opening, closing) => {}
                        (Some(_), _) => {
                            continue 'outer;
                        }
                        (None, _) => panic!()
                    }
                }
            }
        }

        let mut score = 0;
        let mut stack2: Vec<Token> = vec![];

        while let Some(t) = stack.pop() {
            match t {
                ParenClose | SquareClose | CurlyClose | AngleClose => { stack2.push(*t); }
                ParenOpen | SquareOpen | CurlyOpen | AngleOpen => {
                    match (t, stack2.pop()) {
                        (opening, Some(closing)) if Token::matches(opening, &closing) => {},
                        (opening, None) => {
                            score = score * 5 + opening.score();
                        }
                        _ => panic!()
                    }
                },
            }
        }

        scores.push(score);
    }

    scores.sort();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(26397, part1(&parse(include_str!("../input/2021/day10.part1.test.26397.txt")).unwrap()));
    }

    #[test]
    fn part2_example() {
        assert_eq!(288957, part2(&parse(include_str!("../input/2021/day10.part2.test.288957.txt")).unwrap()));
    }
}