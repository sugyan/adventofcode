use aoc2024::{Solve, run};
use std::{
    io::{BufRead, BufReader, Read},
    str::FromStr,
};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
    #[error("invalid line")]
    InvalidLine,
}

#[derive(Debug)]
struct CalibrationEquation {
    test_value: u64,
    numbers: Vec<u64>,
}

impl CalibrationEquation {
    fn calibration_result(&self, third_operator: bool) -> Option<u64> {
        if self.is_possible(self.numbers[0], &self.numbers[1..], third_operator) {
            Some(self.test_value)
        } else {
            None
        }
    }
    fn is_possible(&self, current: u64, numbers: &[u64], third_operator: bool) -> bool {
        match numbers {
            [] => current == self.test_value,
            [n, rest @ ..] => {
                (third_operator
                    && self.is_possible(
                        current * 10_u64.pow(n.ilog10() + 1) + n,
                        rest,
                        third_operator,
                    ))
                    || self.is_possible(current + n, rest, third_operator)
                    || self.is_possible(current * n, rest, third_operator)
            }
        }
    }
}

impl FromStr for CalibrationEquation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(": ")
            .ok_or(Error::InvalidLine)
            .and_then(|(test_value, numbers)| {
                Ok(Self {
                    test_value: test_value.parse()?,
                    numbers: numbers
                        .split_ascii_whitespace()
                        .map(str::parse)
                        .collect::<Result<_, _>>()?,
                })
            })
    }
}

struct Solution {
    calibration_equations: Vec<CalibrationEquation>,
}

impl Solve for Solution {
    type Answer1 = u64;
    type Answer2 = u64;
    type Error = Error;

    fn new<R>(r: R) -> Result<Self, Error>
    where
        R: Read,
    {
        Ok(Self {
            calibration_equations: BufReader::new(r)
                .lines()
                .map(|line| line?.parse())
                .collect::<Result<_, _>>()?,
        })
    }
    fn part1(&self) -> Self::Answer1 {
        self.calibration_equations
            .iter()
            .filter_map(|eq| eq.calibration_result(false))
            .sum()
    }
    fn part2(&self) -> Self::Answer2 {
        self.calibration_equations
            .iter()
            .filter_map(|eq| eq.calibration_result(true))
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
        r"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part1(), 3749);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part2(), 11387);
        Ok(())
    }
}
