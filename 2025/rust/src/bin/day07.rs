use aoc2025::{Day, run};
use itertools::Itertools;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("invalid input")]
    InvalidInput,
}

struct Input {
    grid: Vec<Vec<bool>>,
    start: usize,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect_vec();
        let grid = lines
            .iter()
            .map(|line| line.chars().map(|c| c == '^').collect_vec())
            .collect_vec();
        let start = lines[0]
            .chars()
            .position(|c| c == 'S')
            .ok_or(Error::InvalidInput)?;
        Ok(Self { grid, start })
    }
}

struct Solution;

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = u32;
    type Answer2 = u32;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        let mut beams = vec![false; input.grid[0].len()];
        beams[input.start] = true;
        let mut count = 0;
        for row in &input.grid {
            for (i, col) in row.iter().enumerate() {
                if *col && beams[i] {
                    count += 1;
                    beams[i - 1] = true;
                    beams[i] = false;
                    beams[i + 1] = true;
                }
            }
        }
        count
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
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"
        .trim_start()
        .parse()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::part1(&example_input()?), 21);
        Ok(())
    }
}
