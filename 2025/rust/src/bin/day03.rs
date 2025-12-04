use aoc2025::{Day, run};
use itertools::Itertools;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("invalid digit")]
    InvalidDigit,
}

struct Input(Vec<Vec<u64>>);

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_digit(10).map(u64::from).ok_or(Error::InvalidDigit))
                        .try_collect()
                })
                .try_collect()?,
        ))
    }
}

struct Solution;

impl Solution {
    fn max_n_digit_number(digits: &[u64], n: usize) -> u64 {
        let mut dp = vec![vec![0_u64; digits.len() + 1]; n + 1];
        for i in 1..=n {
            let n = 10_u64.pow(i as u32 - 1);
            for j in (0..=digits.len() - i).rev() {
                dp[i][j] = dp[i][j + 1].max(digits[j] * n + dp[i - 1][j + 1]);
            }
        }
        dp[n][0]
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
            .map(|v| Solution::max_n_digit_number(v, 2))
            .sum()
    }

    fn part2(input: &Self::Input) -> Self::Answer2 {
        input
            .0
            .iter()
            .map(|v| Solution::max_n_digit_number(v, 12))
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
987654321111111
811111111111119
234234234234278
818181911112111
"
        .trim_start()
        .parse()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::part1(&example_input()?), 357);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::part2(&example_input()?), 3_121_910_778_619);
        Ok(())
    }
}
