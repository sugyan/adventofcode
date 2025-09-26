use aoc2024::{Day, run_day};
use itertools::Itertools;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
    #[error("invalid input")]
    InvalidInput,
}

struct Input {
    l: Vec<u32>,
    r: Vec<u32>,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let transposed = s
            .lines()
            .map(|line| {
                line.split_ascii_whitespace()
                    .collect_tuple()
                    .ok_or(Error::InvalidInput)
                    .and_then(|(a, b)| Ok((a.parse()?, b.parse()?)))
            })
            .collect::<Result<Vec<(u32, u32)>, _>>()?
            .into_iter()
            .unzip::<_, _, Vec<_>, Vec<_>>();
        Ok(Self {
            l: transposed.0.into_iter().sorted().collect(),
            r: transposed.1.into_iter().sorted().collect(),
        })
    }
}

struct Solution;

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = u32;
    type Answer2 = u32;

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

fn main() -> Result<(), aoc2024::Error<Error>> {
    run_day::<Solution>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> Result<Input, Error> {
        r"
3   4
4   3
2   5
1   3
3   9
3   3
"
        .trim_start()
        .parse()
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
