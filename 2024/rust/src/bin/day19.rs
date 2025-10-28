use aoc2024::{Day, run_day};
use itertools::Itertools;
use std::{collections::HashSet, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("invalid input")]
    InvalidInput,
}

struct Input {
    patterns: HashSet<String>,
    designs: Vec<String>,
    lengths: Vec<usize>,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().map(String::from).collect_vec();
        let patterns = lines
            .first()
            .ok_or(Error::InvalidInput)?
            .split(", ")
            .map(String::from)
            .collect::<HashSet<_>>();
        let designs = lines.get(2..).ok_or(Error::InvalidInput)?.to_vec();
        let lengths = patterns.iter().map(String::len).unique().collect_vec();
        Ok(Self {
            patterns,
            designs,
            lengths,
        })
    }
}

struct Solution;

impl Solution {
    fn count_paths(input: &Input, target: &str) -> usize {
        let mut counts = vec![0; target.len() + 1];
        counts[0] = 1;
        for i in 0..target.len() {
            for len in &input.lengths {
                let j = i + len;
                if j <= target.len() && input.patterns.contains(&target[i..j]) {
                    counts[j] += counts[i];
                }
            }
        }
        counts[target.len()]
    }
}

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = usize;
    type Answer2 = usize;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        input
            .designs
            .iter()
            .filter(|design| Solution::count_paths(input, design) > 0)
            .count()
    }
    fn part2(input: &Self::Input) -> Self::Answer2 {
        input
            .designs
            .iter()
            .map(|design| Solution::count_paths(input, design))
            .sum()
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
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
"
        .trim_start()
        .parse()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::part1(&example_input()?), 6);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::part2(&example_input()?), 16);
        Ok(())
    }
}
