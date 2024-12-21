use aoc2024::{run, Solve};
use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::Display,
    io::{BufRead, BufReader, Read},
};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Debug)]
enum Sequence {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

impl Display for Sequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Up(i) => write!(f, "{}", vec!["^"; *i].join("")),
            Self::Down(i) => write!(f, "{}", vec!["v"; *i].join("")),
            Self::Left(j) => write!(f, "{}", vec!["<"; *j].join("")),
            Self::Right(j) => write!(f, "{}", vec![">"; *j].join("")),
        }
    }
}

struct Solution {
    codes: Vec<String>,
}

impl Solution {
    fn build_patterns(
        keys: HashMap<(usize, usize), Option<char>>,
    ) -> HashMap<(char, char), Vec<Vec<Sequence>>> {
        let mut patterns = HashMap::new();
        for ((i0, j0), src) in keys.iter() {
            for ((i1, j1), dst) in keys.iter() {
                let (Some(src), Some(dst)) = (src, dst) else {
                    continue;
                };
                use Sequence::*;
                let candidates = match (i0.cmp(i1), j0.cmp(j1)) {
                    (Ordering::Less, Ordering::Less) => vec![
                        vec![Down(i1 - i0), Right(j1 - j0)],
                        vec![Right(j1 - j0), Down(i1 - i0)],
                    ],
                    (Ordering::Less, Ordering::Equal) => vec![vec![Down(i1 - i0)]],
                    (Ordering::Less, Ordering::Greater) => vec![
                        vec![Down(i1 - i0), Left(j0 - j1)],
                        vec![Left(j0 - j1), Down(i1 - i0)],
                    ],
                    (Ordering::Equal, Ordering::Less) => vec![vec![Right(j1 - j0)]],
                    (Ordering::Equal, Ordering::Equal) => vec![Vec::new()],
                    (Ordering::Equal, Ordering::Greater) => vec![vec![Left(j0 - j1)]],
                    (Ordering::Greater, Ordering::Less) => vec![
                        vec![Up(i0 - i1), Right(j1 - j0)],
                        vec![Right(j1 - j0), Up(i0 - i1)],
                    ],
                    (Ordering::Greater, Ordering::Equal) => vec![vec![Up(i0 - i1)]],
                    (Ordering::Greater, Ordering::Greater) => vec![
                        vec![Up(i0 - i1), Left(j0 - j1)],
                        vec![Left(j0 - j1), Up(i0 - i1)],
                    ],
                };
                patterns.insert(
                    (*src, *dst),
                    candidates
                        .into_iter()
                        .filter(|sequences| {
                            sequences.first().map_or(true, |s| {
                                let (i, j) = match s {
                                    Up(i) => (i0 - i, *j0),
                                    Down(i) => (i0 + i, *j0),
                                    Left(j) => (*i0, j0 - j),
                                    Right(j) => (*i0, j0 + j),
                                };
                                keys.get(&(i, j)).map_or(true, |c| c.is_some())
                            })
                        })
                        .collect(),
                );
            }
        }
        patterns
    }
    fn translate(s: &str, patterns: &HashMap<(char, char), Vec<Vec<Sequence>>>) -> Vec<String> {
        let mut v = vec![String::new()];
        for cs in format!("A{s}").chars().tuple_windows() {
            v = v
                .into_iter()
                .flat_map(|s| {
                    patterns[&cs]
                        .iter()
                        .map(|seq| format!("{s}{}", seq.iter().map(ToString::to_string).join("")))
                        .collect::<Vec<_>>()
                })
                .map(|s| format!("{s}A"))
                .collect();
        }
        v
    }
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = u32;
    type Error = Error;

    fn new<R>(r: R) -> Result<Self, Error>
    where
        R: Read,
    {
        Ok(Self {
            codes: BufReader::new(r).lines().collect::<Result<_, _>>()?,
        })
    }
    fn part1(&self) -> Self::Answer1 {
        let key_map = |map: &[Vec<Option<char>>]| {
            map.iter()
                .enumerate()
                .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &c)| ((i, j), c)))
                .collect()
        };
        let patterns = Self::build_patterns(key_map(&[
            vec![Some('7'), Some('8'), Some('9')],
            vec![Some('4'), Some('5'), Some('6')],
            vec![Some('1'), Some('2'), Some('3')],
            vec![None, Some('0'), Some('A')],
        ]))
        .into_iter()
        .chain(Self::build_patterns(key_map(&[
            vec![None, Some('^'), Some('A')],
            vec![Some('<'), Some('v'), Some('>')],
        ])))
        .collect::<HashMap<_, _>>();
        let mut sum = 0;
        for code in &self.codes {
            let mut counts = HashMap::new();
            for keys in Self::translate(code, &patterns)
                .iter()
                .flat_map(|keys| Self::translate(keys, &patterns))
                .flat_map(|keys| Self::translate(&keys, &patterns))
            {
                *counts.entry(keys.len()).or_insert(0) += 1;
            }
            if let Some(min) = counts.keys().min() {
                sum += min
                    * code
                        .strip_suffix('A')
                        .and_then(|s| s.parse::<usize>().ok())
                        .expect("invalid code");
            }
        }
        sum
    }
    fn part2(&self) -> Self::Answer2 {
        todo!()
    }
}

fn main() -> Result<(), Error> {
    run::<Solution>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static [u8] {
        r"
029A
980A
179A
456A
379A
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part1(), 126384);
        Ok(())
    }
}
