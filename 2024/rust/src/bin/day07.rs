use aoc2024::{Day, run_day};
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
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

struct Input(Vec<CalibrationEquation>);

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()
            .map(Self)
    }
}

struct Solution;

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = u64;
    type Answer2 = u64;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        input
            .0
            .iter()
            .filter_map(|eq| eq.calibration_result(false))
            .sum()
    }
    fn part2(input: &Self::Input) -> Self::Answer2 {
        input
            .0
            .iter()
            .filter_map(|eq| eq.calibration_result(true))
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
            .parse()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::part1(&example_input()?), 3749);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::part2(&example_input()?), 11387);
        Ok(())
    }
}
