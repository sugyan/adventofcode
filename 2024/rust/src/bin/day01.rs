use aoc2024::{run, Solve};
use itertools::Itertools;
use std::io::{BufRead, BufReader, Read};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("invalid input")]
    InvalidInput,
    #[error("invalid number")]
    Parse(#[from] std::num::ParseIntError),
}

struct Solution {
    pairs: Vec<(i32, i32)>,
}

impl Solve for Solution {
    type Answer1 = i32;
    type Answer2 = i32;
    type Error = Error;

    fn new(r: impl Read) -> Result<Self, Self::Error> {
        Ok(Self {
            pairs: BufReader::new(r)
                .lines()
                .map(|line| {
                    line.map_err(Error::Io)?
                        .split_ascii_whitespace()
                        .collect_tuple()
                        .ok_or(Error::InvalidInput)
                        .and_then(|(a, b)| Ok((a.parse()?, b.parse()?)))
                })
                .collect::<Result<_, _>>()?,
        })
    }
    fn part1(&self) -> Self::Answer1 {
        let mut l = self.pairs.iter().map(|(l, _)| l).collect::<Vec<_>>();
        let mut r = self.pairs.iter().map(|(_, r)| r).collect::<Vec<_>>();
        l.sort_unstable();
        r.sort_unstable();
        l.into_iter().zip(r).map(|(l, r)| (r - l).abs()).sum()
    }
    fn part2(&self) -> Self::Answer2 {
        todo!()
    }
}

fn main() {
    run::<Solution>();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static [u8] {
        r"
3   4
4   3
2   5
1   3
3   9
3   3
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part1(), 11);
        Ok(())
    }
}
