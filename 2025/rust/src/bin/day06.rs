use aoc2025::{Day, run};
use itertools::{Itertools, izip};
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
    #[error("invalid operator")]
    InvalidOperator,
}

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

struct Input {
    numbers: Vec<Vec<u64>>,
    operators: Vec<Operator>,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();
        Ok(Self {
            numbers: lines[0..lines.len() - 1]
                .iter()
                .map(|line| line.split_ascii_whitespace().map(str::parse).try_collect())
                .try_collect()?,
            operators: lines[lines.len() - 1]
                .split_ascii_whitespace()
                .map(|s| match s {
                    "+" => Ok(Operator::Add),
                    "*" => Ok(Operator::Multiply),
                    _ => Err(Error::InvalidOperator),
                })
                .try_collect()?,
        })
    }
}

struct Solution;

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = u64;
    type Answer2 = u32;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        input
            .numbers
            .iter()
            .skip(1)
            .fold(input.numbers[0].clone(), |acc, v| {
                izip!(acc, v, &input.operators)
                    .map(|(a, b, op)| match op {
                        Operator::Add => a + b,
                        Operator::Multiply => a * b,
                    })
                    .collect()
            })
            .iter()
            .sum()
    }

    fn part2(_: &Self::Input) -> Self::Answer2 {
        todo!()
    }
}

fn main() -> Result<(), aoc2025::Error<Error>> {
    run::<Solution>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> Result<Input, Error> {
        r"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
"
        .trim_start()
        .parse()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::part1(&example_input()?), 4_277_556);
        Ok(())
    }
}
