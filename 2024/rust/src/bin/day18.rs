use aoc2024::{Solve, run};
use itertools::Itertools;
use std::{
    collections::VecDeque,
    io::{BufRead, BufReader, Read},
};
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
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
    #[error("invalid line")]
    InvalidLine,
}
struct Solution {
    positions: Vec<(usize, usize)>,
}

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

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = String;
    type Error = Error;

    fn new<R>(r: R) -> Result<Self, Error>
    where
        R: Read,
    {
        Ok(Self {
            positions: BufReader::new(r)
                .lines()
                .map(|line| {
                    line?
                        .split_once(',')
                        .ok_or(Error::InvalidLine)
                        .and_then(|(x, y)| Ok((x.parse()?, y.parse()?)))
                })
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
    fn part1(&self) -> Self::Answer1 {
        let mut space = vec![vec![true; SIZE]; SIZE];
        for (x, y) in self.positions.iter().take(FIRST_SOME_BYTES) {
            space[*y][*x] = false;
        }
        Solution::bfs(&space).unwrap()
    }
    fn part2(&self) -> Self::Answer2 {
        let mut space = vec![vec![true; SIZE]; SIZE];
        for (x, y) in &self.positions {
            space[*y][*x] = false;
            if Self::bfs(&space).is_none() {
                return format!("{x},{y}");
            }
        }
        unreachable!()
    }
}

fn main() -> Result<(), Error> {
    run::<Solution>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static [u8] {
        &r"
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
        .as_bytes()[1..]
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part1(), 22);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part2(), "6,1");
        Ok(())
    }
}
