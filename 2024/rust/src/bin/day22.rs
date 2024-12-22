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

struct Generator {
    secret_number: u64,
}

impl Generator {
    const MOD: u64 = 16777216;
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.secret_number = ((self.secret_number * 64) ^ self.secret_number) % Self::MOD;
        self.secret_number = ((self.secret_number / 32) ^ self.secret_number) % Self::MOD;
        self.secret_number = ((self.secret_number * 2048) ^ self.secret_number) % Self::MOD;
        Some(self.secret_number)
    }
}

struct Solution {
    secret_numbers: Vec<u64>,
}

impl Solve for Solution {
    type Answer1 = u64;
    type Answer2 = u64;
    type Error = Error;

    fn new<R>(r: R) -> Result<Self, Error>
    where
        R: Read,
    {
        Ok(Self {
            secret_numbers: BufReader::new(r)
                .lines()
                .map(|line| {
                    line.map_err(Error::Io)
                        .and_then(|s| s.parse().map_err(Error::Parse))
                })
                .collect::<Result<_, _>>()?,
        })
    }
    fn part1(&self) -> Self::Answer1 {
        self.secret_numbers
            .iter()
            .filter_map(|&secret_number| Generator { secret_number }.nth(1999))
            .sum()
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
1
10
100
2024
"[1..]
            .as_bytes()
    }

    #[test]
    fn generator() {
        let mut gen = Generator { secret_number: 123 };
        assert_eq!(gen.next(), Some(15887950));
        assert_eq!(gen.next(), Some(16495136));
        assert_eq!(gen.next(), Some(527345));
        assert_eq!(gen.next(), Some(704524));
        assert_eq!(gen.next(), Some(1553684));
        assert_eq!(gen.next(), Some(12683156));
        assert_eq!(gen.next(), Some(11100544));
        assert_eq!(gen.next(), Some(12249484));
        assert_eq!(gen.next(), Some(7753432));
        assert_eq!(gen.next(), Some(5908254));
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part1(), 37327623);
        Ok(())
    }
}
