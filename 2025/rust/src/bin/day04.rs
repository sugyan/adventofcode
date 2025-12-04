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

impl Solution {
    fn removable_rolls(grid: &[Vec<bool>]) -> Vec<(usize, usize)> {
        let (rows, cols) = (grid.len(), grid[0].len());
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
        let mut removable = vec![];
        for i in 0..rows {
            for j in 0..cols {
                if !grid[i][j] {
                    continue;
                }
                if d.iter()
                    .filter_map(|(di, dj)| {
                        let ni = i.wrapping_add(*di);
                        let nj = j.wrapping_add(*dj);
                        if ni < rows && nj < cols {
                            Some((ni, nj))
                        } else {
                            None
                        }
                    })
                    .filter(|&(ni, nj)| grid[ni][nj])
                    .count()
                    < 4
                {
                    removable.push((i, j));
                }
            }
        }
        removable
    }
}

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = usize;
    type Answer2 = usize;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        Self::removable_rolls(&input.0).len()
    }

    fn part2(input: &Self::Input) -> Self::Answer2 {
        let mut grid = input.0.clone();
        let mut total = 0;
        loop {
            let removable = Self::removable_rolls(&grid);
            if removable.is_empty() {
                return total;
            }
            total += removable.len();
            for (i, j) in removable {
                grid[i][j] = false;
            }
        }
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
