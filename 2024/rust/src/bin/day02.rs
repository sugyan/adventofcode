use aoc2024::{run, Solve};
use itertools::Itertools;
use std::{
    io::{BufRead, BufReader, Read},
    iter,
};
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

impl Solution {
    fn is_safe(report: &[u32]) -> bool {
        report.windows(2).map(|w| w[0].cmp(&w[1])).all_equal()
            && report
                .windows(2)
                .all(|w| (1..=3).contains(&w[0].abs_diff(w[1])))
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
        self.reports.iter().filter(|&r| Self::is_safe(r)).count()
    }
    fn part2(&self) -> Self::Answer2 {
        self.reports
            .iter()
            .filter(|&r| {
                (0..r.len())
                    .map(|i| [&r[0..i], &r[i + 1..]].concat())
                    .chain(iter::once(r.clone()))
                    .any(|r| Self::is_safe(&r))
            })
            .count()
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

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part2(), 4);
        Ok(())
    }
}
