use aoc2024::{Solve, run};
use itertools::Itertools;
use std::io::{BufRead, BufReader, Read};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
    #[error("invalid input")]
    InvalidInput,
}

struct Solution {
    sorted_pairs: (Vec<u32>, Vec<u32>),
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;
    type Error = Error;

    fn new<R>(r: R) -> Result<Self, Error>
    where
        R: Read,
    {
        let transposed = BufReader::new(r)
            .lines()
            .map(|line| {
                line.map_err(Error::Io)?
                    .split_ascii_whitespace()
                    .collect_tuple()
                    .ok_or(Error::InvalidInput)
                    .and_then(|(a, b)| Ok((a.parse()?, b.parse()?)))
            })
            .collect::<Result<Vec<(u32, u32)>, _>>()?
            .into_iter()
            .unzip::<_, _, Vec<_>, Vec<_>>();
        Ok(Self {
            sorted_pairs: (
                transposed.0.into_iter().sorted().collect(),
                transposed.1.into_iter().sorted().collect(),
            ),
        })
    }
    fn part1(&self) -> Self::Answer1 {
        self.sorted_pairs
            .0
            .iter()
            .zip(&self.sorted_pairs.1)
            .map(|(l, r)| l.abs_diff(*r))
            .sum()
    }
    fn part2(&self) -> Self::Answer2 {
        let counts = self.sorted_pairs.1.iter().counts();
        self.sorted_pairs
            .0
            .iter()
            .counts()
            .iter()
            .map(|(&k, v)| k * (v * counts.get(k).copied().unwrap_or_default()) as u32)
            .sum()
    }
}

fn main() -> Result<(), Error> {
    run::<Solution>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static [u8] {
        &r"
3   4
4   3
2   5
1   3
3   9
3   3
"
        .as_bytes()[1..]
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part1(), 11);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part2(), 31);
        Ok(())
    }
}
