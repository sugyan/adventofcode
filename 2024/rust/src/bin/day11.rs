use aoc2024::{run, Solve};
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
        let mut stones = self.stones.clone();
        for _ in 0..25 {
            stones = stones
                .iter()
                .flat_map(|n| {
                    if *n == 0 {
                        return vec![1];
                    }
                    let digits = n.ilog10() + 1;
                    if digits % 2 == 0 {
                        let d = 10_u64.pow(digits / 2);
                        vec![n / d, n % d]
                    } else {
                        vec![n * 2024]
                    }
                })
                .collect();
        }
        stones.len()
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
