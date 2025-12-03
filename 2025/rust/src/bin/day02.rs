use aoc2025::{Day, run};
use itertools::Itertools;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
    #[error("invalid range")]
    InvalidRange,
}

#[derive(Debug)]
struct Range(u64, u64);

impl FromStr for Range {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split('-')
            .collect_tuple()
            .ok_or(Error::InvalidRange)
            .and_then(|(first, last)| Ok(Range(first.parse()?, last.parse()?)))
    }
}

struct Input(Vec<Range>);

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.replace("\n", "")
                .split(',')
                .map(str::parse)
                .collect::<Result<_, _>>()?,
        ))
    }
}

struct Solution;

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = u64;
    type Answer2 = u32;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        let mut sum = 0;
        for r in input.0.iter() {
            for i in r.0..=r.1 {
                let d = i.ilog10();
                if d % 2 == 0 {
                    continue;
                }
                let n = 10_u64.pow(d.div_ceil(2));
                if i / n == i % n {
                    sum += i;
                }
            }
        }
        sum
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
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
"
        .trim_start()
        .parse()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::part1(&example_input()?), 1_227_775_554);
        Ok(())
    }
}
