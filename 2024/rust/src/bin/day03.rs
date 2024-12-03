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
        (0..self.memory.len())
            .filter_map(|i| {
                if self.memory[i..].starts_with("mul(") {
                    let j = self.memory[i + 4..].find(')')?;
                    let (a, b) = self.memory[i + 4..i + 4 + j]
                        .split(',')
                        .map(|s| s.parse::<u32>().ok().filter(|n| *n < 1000))
                        .collect::<Option<Vec<_>>>()?
                        .into_iter()
                        .collect_tuple()?;
                    Some(a * b)
                } else {
                    None
                }
            })
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
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part1(), 161);
        Ok(())
    }
}
