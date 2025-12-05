use aoc2025::{Day, run};
use itertools::Itertools;
use std::{ops::RangeInclusive, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
    #[error("invalid input")]
    InvalidInput,
    #[error("invalid range")]
    InvalidRange,
}

struct Input {
    ranges: Vec<RangeInclusive<u64>>,
    ids: Vec<u64>,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .map(String::from)
            .collect_vec()
            .split(String::is_empty)
            .collect_tuple()
            .ok_or(Error::InvalidInput)
            .and_then(|(range_lines, id_lines)| {
                let ranges = range_lines
                    .iter()
                    .map(|line| {
                        line.split_once('-')
                            .ok_or(Error::InvalidRange)
                            .and_then(|(start, end)| {
                                Ok(RangeInclusive::new(start.parse()?, end.parse()?))
                            })
                    })
                    .try_collect()?;
                let ids = id_lines.iter().map(|line| line.parse()).try_collect()?;
                Ok(Self { ranges, ids })
            })
    }
}

struct Solution;

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = usize;
    type Answer2 = u32;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        input
            .ids
            .iter()
            .filter(|id| input.ranges.iter().any(|range| range.contains(id)))
            .count()
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
3-5
10-14
16-20
12-18

1
5
8
11
17
32
"
        .trim_start()
        .parse()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::part1(&example_input()?), 3);
        Ok(())
    }
}
