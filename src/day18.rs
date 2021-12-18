use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use Token::*;


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Token {
    LeftBracket,
    RightBracket,
    Number(i32),
}

type Tokens = Vec<Token>;

fn tokenize(line: &str) -> Tokens {
    lazy_static! {
        static ref TOKEN_PATTERN: Regex = Regex::new(r"(\d+|[\[\],])").unwrap();
    }

    TOKEN_PATTERN.find_iter(line)
        .filter_map(|m| {
            let token = &line[m.start()..m.end()];
            match token {
                "[" => Some(LeftBracket),
                "]" => Some(RightBracket),
                "," => None,
                _ => Some(Number(token.parse().unwrap())),
            }
        })
        .collect()
}

fn explode(tokens: &Tokens) -> Option<Tokens> {
    let depths = tokens
        .iter()
        .fold(vec![], |mut acc, token| {
            acc.push(match token {
                LeftBracket => *acc.last().unwrap_or(&-1) + 1,
                RightBracket => *acc.last().unwrap() - 1,
                Number(_) => *acc.last().unwrap(),
            });

            acc
        });

    if !depths.iter().any(|d| *d >= 4) {
        return None;
    }

    if let Some((i, a, b)) = (4..(tokens.len() - 7)).find_map(|candidate| match (depths[candidate], &tokens[candidate..(candidate + 4)]) {
        (d, &[LeftBracket, Number(a), Number(b), RightBracket]) if d >= 4 => Some((candidate, a, b)),
        _ => None,
    }) {
        let mut left: Tokens = tokens[..i].iter().copied().collect();
        let mut right: Tokens = tokens[(i+4)..].iter().copied().collect();

        if let Some((li, lv)) = left.iter().enumerate().rev().find_map(|candidate| match candidate {
            (li, &Number(v)) => Some((li, v)),
            _ => None,
        }) {
            left[li] = Number(lv + a);
        }

        if let Some((ri, rv)) = right.iter().enumerate().find_map(|candidate| match candidate {
            (ri, &Number(v)) => Some((ri, v)),
            _x => None,
        }) {
            right[ri] = Number(rv + b);
        }

        return Some(
            [ left.as_slice(), &[Number(0)], right.as_slice() ]
                .concat()
                .iter()
                .copied()
                .collect()
        );
    }

    None
}

fn split(tokens: &Tokens) -> Option<Tokens> {
    if let Some((i, &Number(v))) = tokens.iter().enumerate().find(|(_, t)| match t {
        Number(v) => *v >= 10,
        _ => false,
    }) {
        let left: Tokens = tokens[..i].iter().copied().collect();
        let right: Tokens = tokens[(i+1)..].iter().copied().collect();

        return Some(
            [ left.as_slice(), &[ LeftBracket, Number(v / 2), Number((v + 1) / 2), RightBracket ], right.as_slice() ]
                .concat()
                .iter()
                .copied()
                .collect()
        );
    }

    None
}

fn reduce(tokens: &Tokens) -> Tokens {
    let mut result = tokens.clone();

    loop {
        if let Some(ts) = explode(&result) {
            result = ts;
            continue;
        }

        if let Some(ts) = split(&result) {
            result = ts;
            continue;
        }

        break;
    }

    result
}

fn add(a: &Tokens, b: &Tokens) -> Tokens {
    [ &[LeftBracket], a.as_slice(), b.as_slice(), &[RightBracket] ]
        .concat()
        .iter()
        .copied()
        .collect()
}

fn sum(numbers: &Vec<Tokens>) -> Tokens {
    numbers
        .iter()
        .map(reduce)
        .reduce(|acc, ts| reduce(&add(&acc, &ts)))
        .unwrap()
}

fn magnitude(tokens: &Tokens) -> i32 {
    let mut tokens = tokens.clone();

    while let Some((i, a, b)) = (0..(tokens.len() - 3)).find_map(|candidate| match &tokens[candidate..(candidate + 4)] {
        &[LeftBracket, Number(a), Number(b), RightBracket] => Some((candidate, a, b)),
        _ => None,
    }) {
        let left: Tokens = tokens[..i].iter().copied().collect();
        let right: Tokens = tokens[(i+4)..].iter().copied().collect();

        tokens = [ left.as_slice(), &[ Number(3 * a + 2 * b) ], right.as_slice() ]
            .concat()
            .iter()
            .copied()
            .collect();

        if tokens.len() == 1 {
            break;
        }
    }

    assert_eq!(1, tokens.len());

    if let Some(&Number(m)) = tokens.first() {
        return m;
    }

    panic!();
}

#[aoc_generator(day18)]
fn parse(input: &str) -> Vec<Tokens> {
    input
        .lines()
        .map(tokenize)
        .collect()
}

#[aoc(day18, part1)]
fn part1(input: &Vec<Tokens>) -> i32 {
    let tokens: Tokens = sum(input);
    magnitude(&tokens)
}

#[aoc(day18, part2)]
fn part2(input: &Vec<Tokens>) -> i32 {
    (0..input.len()).cartesian_product(0..input.len())
        .filter(|(i,j)| i != j)
        .map(|(i, j)| magnitude(&sum(&vec![input[i].clone(), input[j].clone()])))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_examples() {
        assert_eq!(
            tokenize("[[1,2],[[3,4],5]]"),
            add(&tokenize("[1,2]"), &tokenize("[[3,4],5]"))
        );
        assert_eq!(
            tokenize("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]"),
            add(&tokenize("[[[[4,3],4],4],[7,[[8,4],9]]]"), &tokenize("[1,1]"))
        );
    }

    #[test]
    fn explode_examples() {
        assert_eq!(
            Some(tokenize("[[[[0,9],2],3],4]")),
            explode(&tokenize("[[[[[9,8],1],2],3],4]"))
        );
        assert_eq!(
            Some(tokenize("[7,[6,[5,[7,0]]]]")),
            explode(&tokenize("[7,[6,[5,[4,[3,2]]]]]"))
        );
        assert_eq!(
            Some(tokenize("[[6,[5,[7,0]]],3]")),
            explode(&tokenize("[[6,[5,[4,[3,2]]]],1]"))
        );
        assert_eq!(
            Some(tokenize("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")),
            explode(&tokenize("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"))
        );
        assert_eq!(
            Some(tokenize("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")),
            explode(&tokenize("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"))
        );
        assert_eq!(
            None,
            explode(&tokenize("[[[[0,9],2],3],4]"))
        );
        assert_eq!(
            None,
            explode(&tokenize("[0,9]"))
        );
    }

    #[test]
    fn split_examples() {
        assert_eq!(
            Some(tokenize("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]")),
            split(&tokenize("[[[[0,7],4],[15,[0,13]]],[1,1]]"))
        );
        assert_eq!(
            Some(tokenize("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]")),
            split(&tokenize("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"))
        );
        assert_eq!(
            None,
            split(&tokenize("[[[[0,7],4],[1,[0,3]]],[1,1]]"))
        );
    }

    #[test]
    fn reduce_examples() {
        assert_eq!(
            tokenize("[0,0]"),
            reduce(&tokenize("[0,0]"))
        );
        assert_eq!(
            tokenize("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"),
            reduce(&add(&tokenize("[[[[4,3],4],4],[7,[[8,4],9]]]"), &tokenize("[1,1]")))
        );
        assert_eq!(
            tokenize("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"),
            reduce(&tokenize("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]"))
        );

    }

    #[test]
    fn magnitude_examples() {
        assert_eq!(143, magnitude(&tokenize("[[1,2],[[3,4],5]]")));
        assert_eq!(1384, magnitude(&tokenize("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")));
        assert_eq!(445, magnitude(&tokenize("[[[[1,1],[2,2]],[3,3]],[4,4]]")));
        assert_eq!(791, magnitude(&tokenize("[[[[3,0],[5,3]],[4,4]],[5,5]]")));
        assert_eq!(1137, magnitude(&tokenize("[[[[5,0],[7,4]],[5,5]],[6,6]]")));
        assert_eq!(3488, magnitude(&tokenize("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")));
    }

    #[test]
    fn sum_examples() {
        assert_eq!(
            tokenize("[1,1]"),
            sum(&vec![
                tokenize("[1,1]"),
            ])
        );
        assert_eq!(
            tokenize("[[1,1],[2,2]]"),
            sum(&vec![
                tokenize("[1,1]"),
                tokenize("[2,2]"),
            ])
        );
        assert_eq!(
            tokenize("[[[1,1],[2,2]],[3,3]]"),
            sum(&vec![
                tokenize("[1,1]"),
                tokenize("[2,2]"),
                tokenize("[3,3]"),
            ])
        );
        assert_eq!(
            tokenize("[[[[1,1],[2,2]],[3,3]],[4,4]]"),
            sum(&vec![
                tokenize("[1,1]"),
                tokenize("[2,2]"),
                tokenize("[3,3]"),
                tokenize("[4,4]"),
            ])
        );
        assert_eq!(
            tokenize("[[[[3,0],[5,3]],[4,4]],[5,5]]"),
            sum(&vec![
                tokenize("[1,1]"),
                tokenize("[2,2]"),
                tokenize("[3,3]"),
                tokenize("[4,4]"),
                tokenize("[5,5]"),
            ])
        );
        assert_eq!(
            tokenize("[[[[5,0],[7,4]],[5,5]],[6,6]]"),
            sum(&vec![
                tokenize("[1,1]"),
                tokenize("[2,2]"),
                tokenize("[3,3]"),
                tokenize("[4,4]"),
                tokenize("[5,5]"),
                tokenize("[6,6]"),
            ])
        );
        assert_eq!(
            tokenize("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"),
            sum(&vec![
                tokenize("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]"),
                tokenize("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]"),
                tokenize("[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]"),
                tokenize("[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]"),
                tokenize("[7,[5,[[3,8],[1,4]]]]"),
                tokenize("[[2,[2,2]],[8,[8,1]]]"),
                tokenize("[2,9]"),
                tokenize("[1,[[[9,3],9],[[9,0],[0,7]]]]"),
                tokenize("[[[5,[7,4]],7],1]"),
                tokenize("[[[[4,2],2],6],[8,7]]"),
            ])
        );
    }

    #[test]
    fn tokenize_examples() {
        assert_eq!(
            vec![LeftBracket, Number(1), Number(2), RightBracket],
            tokenize("[1,2]")
        );
        assert_eq!(
            vec![LeftBracket, Number(9), LeftBracket, Number(8), Number(7), RightBracket, RightBracket],
            tokenize("[9,[8,7]]")
        );
        assert_eq!(
            vec![LeftBracket, LeftBracket, Number(1), Number(9), RightBracket, LeftBracket, Number(8), Number(5), RightBracket, RightBracket],
            tokenize("[[1,9],[8,5]]")
        );
    }

    #[test]
    fn part1_example() {
        assert_eq!(4140, part1(&parse(include_str!("../input/2021/day18.part1.test.4140.txt"))));
    }

    #[test]
    fn part2_example() {
        assert_eq!(3993, part2(&parse(include_str!("../input/2021/day18.part2.test.3993.txt"))));
    }
}