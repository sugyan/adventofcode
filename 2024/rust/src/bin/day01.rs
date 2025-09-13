use aoc2024::{Day, run_day};
use itertools::Itertools;
use std::io::BufRead;
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

struct Input {
    l: Vec<u32>,
    r: Vec<u32>,
}

struct Solution;

impl Day for Solution {
    type Input = Input;
    type Answer1 = u32;
    type Answer2 = u32;
    type Error = Error;

    fn parse<R: BufRead>(r: R) -> Result<Self::Input, Self::Error> {
        let transposed = r
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
        Ok(Input {
            l: transposed.0.into_iter().sorted().collect(),
            r: transposed.1.into_iter().sorted().collect(),
        })
    }

    fn part1(input: &Self::Input) -> Self::Answer1 {
        input
            .l
            .iter()
            .zip(&input.r)
            .map(|(l, r)| l.abs_diff(*r))
            .sum()
    }

    fn part2(input: &Self::Input) -> Self::Answer2 {
        let counts = input.r.iter().counts();
        input
            .l
            .iter()
            .counts()
            .iter()
            .map(|(&k, v)| k * (v * counts.get(k).copied().unwrap_or_default()) as u32)
            .sum()
    }
}

fn main() -> Result<(), Error> {
    run_day::<Solution>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> Result<Input, Error> {
        Solution::parse(
            &r"
3   4
4   3
2   5
1   3
3   9
3   3
"
            .as_bytes()[1..],
        )
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::part1(&example_input()?), 11);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::part2(&example_input()?), 31);
        Ok(())
    }
}
