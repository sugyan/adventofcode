use aoc2025::{Day, run};
use itertools::Itertools;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
    #[error("invalid shape")]
    InvalidShape,
    #[error("invalid region")]
    InvalidRegion,
}

struct Input {
    shapes: Vec<[[bool; 3]; 3]>,
    regions: Vec<((usize, usize), Vec<usize>)>,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().map(String::from).collect_vec();
        let parts = lines.split(String::is_empty).collect_vec();
        let shapes = parts[0..parts.len() - 1]
            .iter()
            .map(|lines| {
                lines[1..]
                    .iter()
                    .map(|line| {
                        line.chars()
                            .map(|c| match c {
                                '#' => Ok(true),
                                '.' => Ok(false),
                                _ => Err(Error::InvalidShape),
                            })
                            .collect::<Result<Vec<_>, _>>()?
                            .into_iter()
                            .collect_array()
                            .ok_or(Error::InvalidShape)
                    })
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
                    .collect_array()
                    .ok_or(Error::InvalidShape)
            })
            .try_collect()?;
        let regions = parts[parts.len() - 1]
            .iter()
            .map(|line| {
                line.split_once(": ")
                    .ok_or(Error::InvalidRegion)
                    .and_then(|(wl, quantities)| {
                        Ok((
                            wl.split_once('x')
                                .ok_or(Error::InvalidRegion)
                                .and_then(|(w, l)| Ok((w.parse()?, l.parse()?)))?,
                            quantities
                                .split_ascii_whitespace()
                                .map(str::parse)
                                .try_collect()?,
                        ))
                    })
            })
            .try_collect()?;
        Ok(Self { shapes, regions })
    }
}

struct Solution;

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = usize;
    type Answer2 = u32;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        let mut s = vec![0; input.shapes.len()];
        for (i, shape) in input.shapes.iter().enumerate() {
            s[i] = shape.iter().flatten().filter(|b| **b).count();
        }
        input
            .regions
            .iter()
            .filter(|(wl, quantities)| {
                quantities
                    .iter()
                    .enumerate()
                    .map(|(i, n)| s[i] * n)
                    .sum::<usize>()
                    <= wl.0 * wl.1
            })
            .count()
    }

    fn part2(_: &Self::Input) -> Self::Answer2 {
        todo!()
    }
}

fn main() -> Result<(), aoc2025::Error<Error>> {
    run::<Solution>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> Result<Input, Error> {
        r"
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
"
        .trim_start()
        .parse()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::part1(&example_input()?), 2);
        Ok(())
    }
}
