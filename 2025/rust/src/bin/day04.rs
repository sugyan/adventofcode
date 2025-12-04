use aoc2025::{Day, run};
use itertools::Itertools;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {}

struct Input(Vec<Vec<bool>>);

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|line| line.chars().map(|c| c == '@').collect_vec())
                .collect(),
        ))
    }
}

struct Solution;

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = u32;
    type Answer2 = u32;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        let (rows, cols) = (input.0.len(), input.0[0].len());
        let d = [
            (1, 1),
            (1, 0),
            (1, !0),
            (0, !0),
            (!0, !0),
            (!0, 0),
            (!0, 1),
            (0, 1),
        ];
        let mut count = 0;
        for i in 0..rows {
            for j in 0..cols {
                if !input.0[i][j] {
                    continue;
                }
                let mut c = 0;
                for (di, dj) in &d {
                    let ni = i.wrapping_add(*di);
                    let nj = j.wrapping_add(*dj);
                    if ni >= rows || nj >= cols {
                        continue;
                    }
                    if input.0[ni][nj] {
                        c += 1;
                    }
                }
                if c < 4 {
                    count += 1;
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
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"
        .trim_start()
        .parse()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::part1(&example_input()?), 13);
        Ok(())
    }
}
