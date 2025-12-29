use aoc2025::{Day, run};
use itertools::Itertools;
use std::{collections::HashSet, str::FromStr};
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
            // s.replace("\n", "")
            s.lines()
                .collect::<String>()
                .split(',')
                .map(str::parse)
                .collect::<Result<_, _>>()?,
        ))
    }
}

struct Solution;

impl Solution {
    const TWICE: [(u32, u64); 5] = [
        (1, 11),     // 1-digit x 2: 11, 22, ...
        (2, 101),    // 2-digit x 2: 1010, 1111, ...
        (3, 1001),   // 3-digit x 2: 100100, 101101, ...
        (4, 10001),  // 4-digit x 2: 10001000, 10011001, ...
        (5, 100001), // 5-digit x 2: 100000100000, 100001100001, ...
    ];
    const MORE: [(u32, u64); 6] = [
        (1, 111),       // 1-digit x 3: 111, 222, ...
        (1, 11111),     // 1-digit x 5: 11111, 22222, ...
        (2, 10101),     // 2-digit x 3: 101010, 111111, ...
        (1, 1111111),   // 1-digit x 7: 1111111, 2222222, ...
        (3, 1001001),   // 3-digit x 3: 100100100, 101101101, ...
        (2, 101010101), // 2-digit x 5: 1010101010, 1111111111, ...
    ];
    fn sum_of_invalid_ids(input: &Input, more: bool) -> u64 {
        let mut hs = HashSet::new();
        for (d, step) in Self::TWICE
            .iter()
            .chain(if more { Self::MORE.iter() } else { [].iter() })
        {
            let p = 10_u64.pow(*d);
            for &Range(first, last) in &input.0 {
                let lower = first.next_multiple_of(*step).max(step * (p / 10));
                let upper = last.min(step * (p - 1));
                if lower <= upper {
                    for i in 0..=(upper - lower) / step {
                        hs.insert(lower + i * step);
                    }
                }
            }
        }
        hs.iter().sum()
    }
}

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = u64;
    type Answer2 = u64;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        Self::sum_of_invalid_ids(input, false)
    }
    fn part2(input: &Self::Input) -> Self::Answer2 {
        Self::sum_of_invalid_ids(input, true)
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
