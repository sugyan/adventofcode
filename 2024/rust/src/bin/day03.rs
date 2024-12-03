use aoc2024::{run, Solve};
use itertools::Itertools;
use std::io::{BufReader, Read};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

struct Solution {
    memory: String,
}

impl Solution {
    fn sum_of_multiplications(&self, conditional: bool) -> u32 {
        let (mut sum, mut enabled) = (0, true);
        for i in 0..self.memory.len() {
            if self.memory[i..].starts_with("mul(") {
                if let Some(value) = self.memory[i + 4..].find(')').and_then(|j| {
                    let (a, b) = self.memory[i + 4..i + 4 + j]
                        .split(',')
                        .map(|s| s.parse::<u32>().ok().filter(|n| *n < 1000))
                        .collect::<Option<Vec<_>>>()?
                        .into_iter()
                        .collect_tuple()?;
                    Some(a * b)
                }) {
                    if !conditional || enabled {
                        sum += value;
                    }
                }
            }
            if self.memory[i..].starts_with("do()") {
                enabled = true;
            }
            if self.memory[i..].starts_with("don't()") {
                enabled = false;
            }
        }
        sum
    }
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;
    type Error = Error;

    fn new<R>(r: R) -> Result<Self, Error>
    where
        R: Read,
    {
        let mut memory = String::new();
        BufReader::new(r).read_to_string(&mut memory)?;
        Ok(Self { memory })
    }
    fn part1(&self) -> Self::Answer1 {
        self.sum_of_multiplications(false)
    }
    fn part2(&self) -> Self::Answer2 {
        self.sum_of_multiplications(true)
    }
}

fn main() -> Result<(), Error> {
    run::<Solution>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(
            Solution::new(
                r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
                    .as_bytes()
            )?
            .part1(),
            161
        );
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(
            Solution::new(
                r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
                    .as_bytes()
            )?
            .part2(),
            48
        );
        Ok(())
    }
}
