use aoc2025::{Day, run};
use itertools::Itertools;
use std::{collections::HashSet, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("invalid input")]
    InvalidInput,
}

#[derive(Debug)]
struct Machine {
    indicator_lights: Vec<bool>,
    wiring_schematics: Vec<Vec<u32>>,
    #[allow(dead_code)]
    joltage_requirements: Vec<u32>,
}

impl FromStr for Machine {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect_vec();
        let trim = |s: &str, (prefix, suffix): (char, char)| {
            s.strip_prefix(prefix)
                .and_then(|s| s.strip_suffix(suffix))
                .ok_or(Error::InvalidInput)
                .map(String::from)
        };
        let indicator_lights = trim(parts[0], ('[', ']'))?
            .chars()
            .rev()
            .map(|c| match c {
                '#' => Ok(true),
                '.' => Ok(false),
                _ => Err(Error::InvalidInput),
            })
            .try_collect()?;
        let wiring_schematics = parts[1..parts.len() - 1]
            .iter()
            .map(|part| {
                trim(part, ('(', ')'))?
                    .split(',')
                    .map(str::parse)
                    .try_collect()
                    .map_err(|_| Error::InvalidInput)
            })
            .try_collect()?;
        Ok(Self {
            indicator_lights,
            wiring_schematics,
            joltage_requirements: trim(parts[parts.len() - 1], ('{', '}'))?
                .split(',')
                .map(str::parse)
                .try_collect()?,
        })
    }
}

struct Input(Vec<Machine>);

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.lines().map(str::parse).try_collect()?))
    }
}

struct Solution;

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = u32;
    type Answer2 = u32;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        let mut total = 0;
        for machine in &input.0 {
            let target = machine
                .indicator_lights
                .iter()
                .fold(0, |acc, b| (acc << 1) | if *b { 1 } else { 0 });
            let schematics = machine
                .wiring_schematics
                .iter()
                .map(|schematic| schematic.iter().map(|x| 1 << *x).sum::<u32>())
                .collect_vec();
            let mut hs = schematics.iter().cloned().collect::<HashSet<_>>();
            let mut presses = 1;
            while !hs.contains(&target) {
                presses += 1;
                hs = hs
                    .iter()
                    .cartesian_product(&schematics)
                    .map(|(a, b)| a ^ b)
                    .collect();
            }
            total += presses;
        }
        total
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
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"
        .trim_start()
        .parse()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::part1(&example_input()?), 7);
        Ok(())
    }
}
