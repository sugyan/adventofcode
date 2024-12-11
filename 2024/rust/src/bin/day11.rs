use aoc2024::{run, Solve};
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
        let mut hm = self.stones.iter().copied().counts();
        for _ in 0..blink {
            hm = hm
                .iter()
                .flat_map(|(k, v)| Self::next_stones(*k).map(|n| (n, *v)))
                .into_grouping_map()
                .sum();
        }
        hm.values().sum()
    }
    fn next_stones(n: u64) -> impl Iterator<Item = u64> {
        if n == 0 {
            vec![1].into_iter()
        } else {
            match n.ilog10() + 1 {
                digits if digits % 2 == 0 => {
                    let d = 10_u64.pow(digits / 2);
                    vec![n / d, n % d].into_iter()
                }
                _ => vec![n * 2024].into_iter(),
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
