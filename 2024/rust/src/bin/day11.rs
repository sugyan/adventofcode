use aoc2024::{Day, run_day};
use itertools::{Either, Itertools};
use std::{iter, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
}

struct Input(Vec<u64>);

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split_whitespace()
                .map(str::parse)
                .collect::<Result<_, _>>()?,
        ))
    }
}

struct Solution;

impl Solution {
    fn count_stones(input: &Input, blink: usize) -> usize {
        let mut counts = input.0.iter().copied().counts();
        for _ in 0..blink {
            counts = counts
                .iter()
                .flat_map(|(k, v)| Self::next_stones(*k).map(|n| (n, *v)))
                .into_grouping_map()
                .sum();
        }
        counts.values().sum()
    }
    fn next_stones(n: u64) -> impl Iterator<Item = u64> {
        if n == 0 {
            Either::Left(iter::once(1))
        } else {
            match n.ilog10() + 1 {
                digits if digits % 2 == 0 => {
                    let d = 10_u64.pow(digits / 2);
                    Either::Right([n / d, n % d].into_iter())
                }
                _ => Either::Left(iter::once(n * 2024)),
            }
        }
    }
}

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = usize;
    type Answer2 = usize;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        Self::count_stones(input, 25)
    }
    fn part2(input: &Self::Input) -> Self::Answer2 {
        Self::count_stones(input, 75)
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
125 17
"
        .trim_start()
        .parse()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::part1(&example_input()?), 55312);
        Ok(())
    }
}
