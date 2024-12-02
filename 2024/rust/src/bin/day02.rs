use aoc2024::{run, Solve};
use std::io::{BufRead, BufReader, Read};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
}

struct Solution {
    reports: Vec<Vec<u32>>,
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;
    type Error = Error;

    fn new<R>(r: R) -> Result<Self, Error>
    where
        R: Read,
    {
        Ok(Self {
            reports: BufReader::new(r)
                .lines()
                .map(|line| {
                    line.map_err(Error::Io)?
                        .split_ascii_whitespace()
                        .map(|s| s.parse().map_err(Error::Parse))
                        .collect::<Result<_, _>>()
                })
                .collect::<Result<_, _>>()?,
        })
    }
    fn part1(&self) -> Self::Answer1 {
        self.reports
            .iter()
            .filter(|r: &&Vec<u32>| {
                r.windows(2).all(|w| w[0] < w[1] && w[1] < w[0] + 4)
                    || r.windows(2).all(|w| w[1] < w[0] && w[0] < w[1] + 4)
            })
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
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part1(), 2);
        Ok(())
    }
}
