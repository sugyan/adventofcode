use aoc2025::{Day, run};
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {}

struct Input(Vec<String>);

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.lines().map(String::from).collect()))
    }
}

struct Solution;

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = u32;
    type Answer2 = u32;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        input
            .0
            .iter()
            .map(|line| {
                let digits = line
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<_>>();
                let mut max = 0;
                for i in 0..digits.len() - 1 {
                    for j in i + 1..digits.len() {
                        max = max.max(digits[i] * 10 + digits[j]);
                    }
                }
                max
            })
            .sum()
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
}
