use aoc2025::{Day, run};
use itertools::Itertools;
use std::{collections::VecDeque, str::FromStr};
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

impl Solution {
    const DIRS: [(usize, usize); 8] = [
        (!0, !0),
        (!0, 0),
        (!0, 1),
        (0, !0),
        (0, 1),
        (1, !0),
        (1, 0),
        (1, 1),
    ];

    fn adjacent_counts(grid: &[Vec<bool>]) -> Vec<Vec<Option<usize>>> {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut counts = vec![vec![None; cols]; rows];
        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] {
                    counts[i][j] = Some(
                        Self::DIRS
                            .iter()
                            .filter(|(di, dj)| {
                                let ni = i.wrapping_add(*di);
                                let nj = j.wrapping_add(*dj);
                                ni < rows && nj < cols && grid[ni][nj]
                            })
                            .count(),
                    );
                }
            }
        }
        counts
    }
}

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = usize;
    type Answer2 = usize;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        Self::adjacent_counts(&input.0)
            .iter()
            .flatten()
            .filter(|o| o.is_some_and(|c| c < 4))
            .count()
    }
    fn part2(input: &Self::Input) -> Self::Answer2 {
        let mut counts = Self::adjacent_counts(&input.0);
        let (rows, cols) = (counts.len(), counts[0].len());
        let mut vd = counts
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(j, &o)| o.filter(|&c| c < 4).map(|_| (i, j)))
            })
            .collect::<VecDeque<_>>();
        let mut count = 0;
        while let Some((i, j)) = vd.pop_front() {
            count += 1;
            for (di, dj) in Self::DIRS.iter() {
                let ni = i.wrapping_add(*di);
                let nj = j.wrapping_add(*dj);
                if ni < rows
                    && nj < cols
                    && let Some(c) = counts[ni][nj].as_mut()
                {
                    if *c == 4 {
                        vd.push_back((ni, nj));
                    }
                    *c -= 1;
                }
            }
        }
        count
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

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::part2(&example_input()?), 43);
        Ok(())
    }
}
