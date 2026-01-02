use aoc2025::{Day, run};
use itertools::Itertools;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("invalid operator")]
    InvalidOperator,
}

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

impl Operator {
    fn perform(&self, numbers: impl Iterator<Item = u64>) -> u64 {
        match self {
            Self::Add => numbers.sum(),
            Self::Multiply => numbers.product(),
        }
    }
}

struct Input {
    numbers: Vec<Vec<char>>,
    operators: Vec<Operator>,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();
        Ok(Self {
            numbers: lines[0..lines.len() - 1]
                .iter()
                .map(|line| line.chars().collect_vec())
                .collect(),
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

impl Solution {
    fn transpose<T>(matrix: &[Vec<T>]) -> Vec<Vec<T>>
    where
        T: Copy,
    {
        let (rows, cols) = (matrix.len(), matrix[0].len());
        (0..cols)
            .map(|c| (0..rows).map(|r| matrix[r][c]).collect())
            .collect()
    }
}

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = u64;
    type Answer2 = u64;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        let m = input
            .numbers
            .iter()
            .map(|row| {
                row.iter()
                    .collect::<String>()
                    .split_ascii_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect_vec()
            })
            .collect_vec();
        Self::transpose(&m)
            .into_iter()
            .zip(&input.operators)
            .map(|(row, operator)| operator.perform(row.into_iter()))
            .sum()
    }
    fn part2(input: &Self::Input) -> Self::Answer2 {
        Self::transpose(&input.numbers)
            .iter()
            .map(|row| row.iter().collect::<String>().trim().to_string())
            .collect_vec()
            .split(String::is_empty)
            .collect_vec()
            .iter()
            .map(|group| group.iter().map(|s| s.parse::<u64>().unwrap()))
            .zip(&input.operators)
            .map(|(group, operator)| operator.perform(group))
            .sum()
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

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::part2(&example_input()?), 3_263_827);
        Ok(())
    }
}
