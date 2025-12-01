use aoc2025::{Day, run};
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("invalid rotation")]
    InvalidRotation,
}

#[derive(Debug)]
enum Rotation {
    L(i32),
    R(i32),
}

impl Rotation {
    fn next_point(&self, current: i32) -> i32 {
        match self {
            Rotation::L(n) => current - n,
            Rotation::R(n) => current + n,
        }
        .rem_euclid(100)
    }
}

impl FromStr for Rotation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, num) = s.split_at(1);
        Ok(match (dir, num.parse()) {
            ("L", Ok(n)) => Rotation::L(n),
            ("R", Ok(n)) => Rotation::R(n),
            _ => return Err(Error::InvalidRotation),
        })
    }
}

struct Input(Vec<Rotation>);

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .map(str::parse)
            .collect::<Result<_, _>>()
            .map(Input)
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
            .0
            .iter()
            .scan(50, |state, rot| {
                *state = rot.next_point(*state);
                Some(*state)
            })
            .filter(|p| *p == 0)
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
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
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
