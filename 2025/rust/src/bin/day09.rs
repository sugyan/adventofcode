use aoc2025::{Day, run};
use itertools::Itertools;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
    #[error("invalid input")]
    InvalidInput,
}

struct Input(Vec<(u64, u64)>);

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|line| {
                    line.split(',')
                        .collect_tuple()
                        .ok_or(Error::InvalidInput)
                        .and_then(|(x, y)| Ok((x.parse()?, y.parse()?)))
                })
                .try_collect()?,
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
        input
            .0
            .iter()
            .combinations(2)
            .map(|combination| {
                let ((x0, y0), (x1, y1)) = (combination[0], combination[1]);
                (x0.abs_diff(*x1) + 1) * (y0.abs_diff(*y1) + 1)
            })
            .max()
            .expect("at least one pair exists")
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
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"
        .trim_start()
        .parse()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::part1(&example_input()?), 50);
        Ok(())
    }
}
