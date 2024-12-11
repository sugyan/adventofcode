use aoc2024::{run, Solve};
use std::{
    collections::HashMap,
    io::{BufReader, Read},
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
    stones: Vec<u64>,
}

impl Solution {
    fn count_stones(&self, blink: usize) -> usize {
        let mut hm = HashMap::new();
        for stone in &self.stones {
            *hm.entry(*stone).or_insert(0) += 1;
        }
        for _ in 0..blink {
            hm = hm
                .iter()
                .flat_map(|(k, v)| {
                    (if *k == 0 {
                        vec![1]
                    } else {
                        let digits = k.ilog10() + 1;
                        if digits % 2 == 0 {
                            let d = 10_u64.pow(digits / 2);
                            vec![k / d, k % d]
                        } else {
                            vec![k * 2024]
                        }
                    })
                    .into_iter()
                    .map(|n| (n, *v))
                })
                .fold(HashMap::new(), |mut acc, (k, v)| {
                    *acc.entry(k).or_insert(0) += v;
                    acc
                });
        }
        hm.values().sum()
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
