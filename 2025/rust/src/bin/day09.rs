use aoc2025::{Day, run};
use itertools::Itertools;
use std::{collections::HashMap, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
    #[error("invalid input")]
    InvalidInput,
}

struct Input(Vec<(u64, u64)>);

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|line| {
                    line.split(',')
                        .collect_tuple()
                        .ok_or(Error::InvalidInput)
                        .and_then(|(x, y)| Ok((x.parse()?, y.parse()?)))
                })
                .try_collect()?,
        ))
    }
}

struct Solution;

impl Solution {
    fn make_map(v: &[u64]) -> HashMap<u64, usize> {
        v.iter()
            .sorted()
            .enumerate()
            .map(|(i, x)| (*x, i))
            .collect()
    }
    fn make_grid(
        input: &Input,
        xmap: &HashMap<u64, usize>,
        ymap: &HashMap<u64, usize>,
    ) -> Vec<Vec<bool>> {
        let mut grid = vec![vec![false; ymap.len()]; xmap.len()];
        for ((x0, y0), (x1, y1)) in input.0.iter().circular_tuple_windows() {
            if x0 == x1 {
                #[allow(clippy::needless_range_loop)]
                for y in ymap[y0].min(ymap[y1])..=ymap[y0].max(ymap[y1]) {
                    grid[xmap[x0]][y] = true;
                }
            }
            if y0 == y1 {
                #[allow(clippy::needless_range_loop)]
                for x in xmap[x0].min(xmap[x1])..=xmap[x0].max(xmap[x1]) {
                    grid[x][ymap[y0]] = true;
                }
            }
        }
        grid
    }
    fn dfs(grid: &[Vec<bool>], start: (usize, usize), seen: &mut [Vec<bool>]) {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut stack = vec![start];
        while let Some((x, y)) = stack.pop() {
            if seen[x][y] || grid[x][y] {
                continue;
            }
            seen[x][y] = true;
            for (dx, dy) in [0, 1, 0, !0, 0].into_iter().tuple_windows() {
                let nx = x.wrapping_add(dx);
                let ny = y.wrapping_add(dy);
                if nx < rows && ny < cols {
                    stack.push((nx, ny));
                }
            }
        }
    }
}

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = u64;
    type Answer2 = u64;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        input
            .0
            .iter()
            .combinations(2)
            .map(|combination| {
                let ((x0, y0), (x1, y1)) = (combination[0], combination[1]);
                (x0.abs_diff(*x1) + 1) * (y0.abs_diff(*y1) + 1)
            })
            .max()
            .expect("at least one pair exists")
    }

    fn part2(input: &Self::Input) -> Self::Answer2 {
        let xs = input.0.iter().map(|(x, _)| *x).unique().collect_vec();
        let ys = input.0.iter().map(|(_, y)| *y).unique().collect_vec();
        let (xlen, ylen) = (xs.len(), ys.len());
        let xmap = Self::make_map(&xs);
        let ymap = Self::make_map(&ys);
        let mut grid = Self::make_grid(input, &xmap, &ymap);
        let mut seen = vec![vec![false; grid[0].len()]; grid.len()];
        for p in [(0, 0), (0, ylen - 1), (xlen - 1, 0), (xlen - 1, ylen - 1)] {
            Self::dfs(&grid, p, &mut seen);
        }
        for x in 0..xlen {
            for y in 0..ylen {
                if !seen[x][y] && !grid[x][y] {
                    grid[x][y] = true;
                }
            }
        }
        input
            .0
            .iter()
            .combinations(2)
            .filter_map(|combination| {
                let ((x0, y0), (x1, y1)) = (combination[0], combination[1]);
                #[allow(clippy::needless_range_loop)]
                for x in xmap[x0].min(xmap[x1])..=xmap[x0].max(xmap[x1]) {
                    for y in ymap[y0].min(ymap[y1])..=ymap[y0].max(ymap[y1]) {
                        if !grid[x][y] {
                            return None;
                        }
                    }
                }
                Some((x0.abs_diff(*x1) + 1) * (y0.abs_diff(*y1) + 1))
            })
            .max()
            .expect("at least one value exists")
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
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"
        .trim_start()
        .parse()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::part1(&example_input()?), 50);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::part2(&example_input()?), 24);
        Ok(())
    }
}
