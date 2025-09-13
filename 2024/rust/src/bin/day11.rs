use aoc2024::{Solve, run};
use itertools::Itertools;
use std::io::{BufReader, Read};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
}

struct Solution {
    stones: Vec<u64>,
}

impl Solution {
    fn count_stones(&self, blink: usize) -> usize {
        let mut counts = self.stones.iter().copied().counts();
        for _ in 0..blink {
            counts = counts
                .iter()
                .flat_map(|(k, v)| Self::next_stones(*k).into_iter().map(|n| (n, *v)))
                .into_grouping_map()
                .sum();
        }
        counts.values().sum()
    }
    fn next_stones(n: u64) -> Vec<u64> {
        if n == 0 {
            vec![1]
        } else {
            match n.ilog10() + 1 {
                digits if digits % 2 == 0 => {
                    let d = 10_u64.pow(digits / 2);
                    vec![n / d, n % d]
                }
                _ => vec![n * 2024],
            }
        }
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
        let mut buf = String::new();
        BufReader::new(r).read_to_string(&mut buf)?;
        Ok(Self {
            stones: buf
                .split_whitespace()
                .map(str::parse)
                .collect::<Result<_, _>>()?,
        })
    }
    fn part1(&self) -> Self::Answer1 {
        self.count_stones(25)
    }
    fn part2(&self) -> Self::Answer2 {
        self.count_stones(75)
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
125 17
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part1(), 55312);
        Ok(())
    }
}
