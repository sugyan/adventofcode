use aoc2024::{Day, run_day};
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
}

struct Input(Vec<Vec<u32>>);

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|line| {
                    line.split_ascii_whitespace()
                        .map(|s| s.parse().map_err(Error::Parse))
                        .collect::<Result<_, _>>()
                })
                .collect::<Result<_, _>>()?,
        ))
    }
}

struct Solution;

impl Solution {
    fn is_safe(report: &[u32]) -> bool {
        let increasing = report.windows(2).all(|w| w[0] < w[1]);
        let decreasing = report.windows(2).all(|w| w[0] > w[1]);
        (increasing || decreasing)
            && report
                .windows(2)
                .all(|w| (1..=3).contains(&w[0].abs_diff(w[1])))
    }
}

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = usize;
    type Answer2 = usize;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        input.0.iter().filter(|&r| Self::is_safe(r)).count()
    }
    fn part2(input: &Self::Input) -> Self::Answer2 {
        input
            .0
            .iter()
            .filter(|&r| {
                (0..r.len())
                    .map(|i| [&r[0..i], &r[i + 1..]].concat())
                    .any(|r| Self::is_safe(&r))
            })
            .count()
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
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"
        .trim_start()
        .parse()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::part1(&example_input()?), 2);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::part2(&example_input()?), 4);
        Ok(())
    }
}
