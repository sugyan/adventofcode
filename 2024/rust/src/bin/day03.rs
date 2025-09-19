use aoc2024::{Day, run_day};
use itertools::Itertools;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {}

struct Input(String);

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.lines().join("")))
    }
}

struct Solution;

impl Solution {
    fn sum_of_multiplications(input: &Input, conditional: bool) -> u32 {
        let (mut sum, mut enabled, mut i) = (0, true, 0);
        let bytes = input.0.as_bytes();
        while i < bytes.len() {
            match bytes.get(i..i + 4) {
                Some(b"mul(") if !conditional || enabled => {
                    sum += bytes[i..]
                        .iter()
                        .position(|&b| b == b')')
                        .and_then(|j| {
                            Self::parse_numbers(&bytes[i + 4..i + j])?
                                .into_iter()
                                .collect_tuple()
                                .map(|(x, y)| x * y)
                        })
                        .unwrap_or_default();
                    i += 4;
                }
                Some(b"do()") => {
                    enabled = true;
                    i += 4;
                }
                Some(b"don'") if bytes.get(i + 4..i + 7) == Some(b"t()") => {
                    enabled = false;
                    i += 7;
                }
                _ => i += 1,
            }
        }
        sum
    }
    fn parse_numbers(s: &[u8]) -> Option<Vec<u32>> {
        s.split(|&b| b == b',')
            .map(|s| str::from_utf8(s).ok()?.parse().ok())
            .collect()
    }
}

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = u32;
    type Answer2 = u32;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        Self::sum_of_multiplications(input, false)
    }
    fn part2(input: &Self::Input) -> Self::Answer2 {
        Self::sum_of_multiplications(input, true)
    }
}

fn main() -> Result<(), aoc2024::Error<Error>> {
    run_day::<Solution>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(
            Solution::part1(
                &r"
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"[1..]
                    .parse()?
            ),
            161
        );
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(
            Solution::part2(
                &r"
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"[1..]
                    .parse()?
            ),
            48
        );
        Ok(())
    }
}
