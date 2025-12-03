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

impl Solution {
    fn is_digits_repeated_twice(d: &u64) -> bool {
        let n = 10_u64.pow(d.ilog10().div_ceil(2));
        d / n == d % n
    }
    fn is_digits_repeated_at_least_twice(d: &u64) -> bool {
        'outer: for i in 1..=d.ilog10().div_ceil(2) {
            let n = 10_u64.pow(i);
            let s = d % n;
            if s == 0 || s.ilog10() + 1 != i {
                continue;
            }
            let mut m = *d;
            while m > 0 {
                if m % n != s {
                    continue 'outer;
                }
                m /= n;
            }
            return true;
        }
        false
    }
}

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = u64;
    type Answer2 = u64;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        input
            .0
            .iter()
            .flat_map(|r| r.0..=r.1)
            .filter(Self::is_digits_repeated_twice)
            .sum()
    }

    fn part2(input: &Self::Input) -> Self::Answer2 {
        input
            .0
            .iter()
            .flat_map(|r| r.0..=r.1)
            .filter(Self::is_digits_repeated_at_least_twice)
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

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::part2(&example_input()?), 4_174_379_265);
        Ok(())
    }
}
