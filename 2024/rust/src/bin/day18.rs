use aoc2024::{Day, run_day};
use itertools::Itertools;
use std::{collections::VecDeque, str::FromStr};
use thiserror::Error;

#[cfg(not(test))]
const SIZE: usize = 71;
#[cfg(test)]
const SIZE: usize = 7;

#[cfg(not(test))]
const FIRST_SOME_BYTES: usize = 1024;
#[cfg(test)]
const FIRST_SOME_BYTES: usize = 12;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
    #[error("invalid line")]
    InvalidLine,
}

struct Input(Vec<(usize, usize)>);

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|line| {
                    line.split_once(',')
                        .ok_or(Error::InvalidLine)
                        .and_then(|(x, y)| Ok((x.parse()?, y.parse()?)))
                })
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

struct Solution;

impl Solution {
    fn bfs(space: &[Vec<bool>]) -> Option<u32> {
        let mut mins = vec![vec![None; SIZE]; SIZE];
        let mut vd = [(0, 0)].into_iter().collect::<VecDeque<(usize, usize)>>();
        mins[0][0] = Some(0);
        while let Some((i, j)) = vd.pop_front() {
            for (di, dj) in [0, 1, 0, !0, 0].iter().tuple_windows() {
                let (ni, nj) = (i.wrapping_add(*di), j.wrapping_add(*dj));
                if (0..SIZE).contains(&ni)
                    && (0..SIZE).contains(&nj)
                    && space[ni][nj]
                    && mins[ni][nj].is_none()
                {
                    mins[ni][nj] = mins[i][j].map(|x| x + 1);
                    vd.push_back((ni, nj));
                }
            }
        }
        mins[SIZE - 1][SIZE - 1]
    }
}

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = u32;
    type Answer2 = String;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        let mut space = vec![vec![true; SIZE]; SIZE];
        for (x, y) in input.0.iter().take(FIRST_SOME_BYTES) {
            space[*y][*x] = false;
        }
        Solution::bfs(&space).unwrap()
    }
    fn part2(input: &Self::Input) -> Self::Answer2 {
        let mut space = vec![vec![true; SIZE]; SIZE];
        for (x, y) in &input.0 {
            space[*y][*x] = false;
            if Self::bfs(&space).is_none() {
                return format!("{x},{y}");
            }
        }
        unreachable!()
    }
}

fn main() -> Result<(), aoc2024::Error<Error>> {
    run_day::<Solution>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> Result<Input, Error> {
        r"
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
"
        .trim_start()
        .parse()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::part1(&example_input()?), 22);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::part2(&example_input()?), "6,1");
        Ok(())
    }
}
