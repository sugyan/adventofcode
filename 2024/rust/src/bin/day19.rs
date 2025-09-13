use aoc2024::{Solve, run};
use std::{
    collections::HashSet,
    io::{BufRead, BufReader, Read},
};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("invalid input")]
    InvalidInput,
}

struct Solution {
    patterns: HashSet<String>,
    max_pattern_len: usize,
    designs: Vec<String>,
}

impl Solution {
    fn count_paths(&self, target: &str) -> usize {
        let mut counts = vec![0; target.len() + 1];
        counts[0] = 1;
        for i in 0..target.len() {
            for j in i..=i + self.max_pattern_len {
                if j <= target.len() && self.patterns.contains(&target[i..j]) {
                    counts[j] += counts[i];
                }
            }
        }
        counts[target.len()]
    }
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;
    type Error = Error;

    fn new<R>(r: R) -> Result<Self, Error>
    where
        R: Read,
    {
        let lines = BufReader::new(r).lines().collect::<Result<Vec<_>, _>>()?;
        let patterns = lines
            .first()
            .ok_or(Error::InvalidInput)?
            .split(", ")
            .map(String::from)
            .collect::<HashSet<_>>();
        let max_pattern_len = patterns.iter().map(String::len).max().unwrap_or(0);
        Ok(Self {
            patterns,
            max_pattern_len,
            designs: lines.get(2..).ok_or(Error::InvalidInput)?.to_vec(),
        })
    }
    fn part1(&self) -> Self::Answer1 {
        self.designs
            .iter()
            .filter(|design| self.count_paths(design) > 0)
            .count()
    }
    fn part2(&self) -> Self::Answer2 {
        self.designs
            .iter()
            .map(|design| self.count_paths(design))
            .sum()
    }
}

fn main() -> Result<(), Error> {
    run::<Solution>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static [u8] {
        &r"
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
        .as_bytes()[1..]
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part1(), 6);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part2(), 16);
        Ok(())
    }
}
