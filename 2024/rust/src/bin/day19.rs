use aoc2024::{run, Solve};
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
    patterns: Vec<String>,
    designs: Vec<String>,
}

impl Solution {
    fn search(&self, target: &str, memo: &mut HashSet<String>) -> bool {
        if target.is_empty() {
            return true;
        } else if memo.contains(target) {
            return false;
        }
        for pattern in &self.patterns {
            if target.starts_with(pattern) && self.search(&target[pattern.len()..], memo) {
                return true;
            }
        }
        memo.insert(target.to_string());
        false
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
        Ok(Self {
            patterns: lines[0].split(", ").map(str::to_string).collect(),
            designs: lines.get(2..).ok_or(Error::InvalidInput)?.to_vec(),
        })
    }
    fn part1(&self) -> Self::Answer1 {
        self.designs
            .iter()
            .filter(|design| self.search(design, &mut HashSet::new()))
            .count()
    }
    fn part2(&self) -> Self::Answer2 {
        todo!()
    }
}

fn main() -> Result<(), Error> {
    run::<Solution>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static [u8] {
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
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part1(), 6);
        Ok(())
    }
}
