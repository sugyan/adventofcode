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
        for i in 0..self.memory.len() - 4 {
            let s = &self.memory[i..];
            match &s[..4] {
                "mul(" if !conditional || enabled => {
                    sum += s
                        .find(')')
                        .and_then(|j| {
                            s[4..j]
                                .split(',')
                                .map(|s| s.parse().ok())
                                .collect::<Option<Vec<u32>>>()?
                                .into_iter()
                                .collect_tuple()
                                .map(|(x, y)| x * y)
                        })
                        .unwrap_or_default()
                }
                "do()" => {
                    enabled = true;
                }
                "don'" if s.starts_with("don't()") => {
                    enabled = false;
                }
                _ => {}
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
        Ok(Self {
            memory: memory.trim().to_string(),
        })
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
                r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
                "
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
