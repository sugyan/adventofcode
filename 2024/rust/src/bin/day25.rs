use aoc2024::{run, Solve};
use itertools::Itertools;
use std::io::{BufRead, BufReader, Read};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("invalid input")]
    InvalidInput,
}

struct Solution {
    locks: Vec<u32>,
    keys: Vec<u32>,
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = String;
    type Error = Error;

    fn new<R>(r: R) -> Result<Self, Error>
    where
        R: Read,
    {
        let (mut locks, mut keys) = (Vec::new(), Vec::new());
        for lines in BufReader::new(r)
            .lines()
            .collect::<Result<Vec<_>, _>>()?
            .split(String::is_empty)
        {
            let value = lines
                .get(1..)
                .ok_or(Error::InvalidInput)?
                .iter()
                .flat_map(|row| row.chars())
                .try_fold(0, |acc, c| {
                    Ok((acc << 1)
                        | match c {
                            '#' => 1,
                            '.' => 0,
                            _ => return Err(Error::InvalidInput),
                        })
                })?;
            if lines.first().ok_or(Error::InvalidInput)? == "#####" {
                locks.push(value);
            } else {
                keys.push(value);
            }
        }
        Ok(Self { locks, keys })
    }
    fn part1(&self) -> Self::Answer1 {
        self.locks
            .iter()
            .cartesian_product(self.keys.iter())
            .filter(|(lock, key)| *lock & *key == 0)
            .count()
    }
    fn part2(&self) -> Self::Answer2 {
        Default::default()
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
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part1(), 3);
        Ok(())
    }
}
