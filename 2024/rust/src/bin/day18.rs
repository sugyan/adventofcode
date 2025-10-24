use aoc2024::{Day, run_day};
use itertools::Itertools;
use std::{
    collections::{BinaryHeap, VecDeque},
    str::FromStr,
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
    Parse(#[from] std::num::ParseIntError),
    #[error("invalid line")]
    InvalidLine,
}

struct Input {
    bytes: Vec<(usize, usize)>,
    space: Vec<Vec<usize>>,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s
            .lines()
            .map(|line| {
                line.split_once(',')
                    .ok_or(Error::InvalidLine)
                    .and_then(|(x, y)| Ok((x.parse()?, y.parse()?)))
            })
            .collect::<Result<Vec<(usize, usize)>, _>>()?;
        let mut space = vec![vec![usize::MAX; SIZE]; SIZE];
        for (i, (x, y)) in bytes.iter().enumerate() {
            space[*y][*x] = i;
        }
        Ok(Self { bytes, space })
    }
}

struct Solution;

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = u32;
    type Answer2 = String;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        let mut dist = vec![vec![None; SIZE]; SIZE];
        let mut vd = VecDeque::from_iter([(0_usize, 0_usize)]);
        dist[0][0] = Some(0);
        while let Some((i, j)) = vd.pop_front() {
            for (di, dj) in [0, 1, 0, !0, 0].iter().tuple_windows() {
                let (ni, nj) = (i.wrapping_add(*di), j.wrapping_add(*dj));
                if ni < SIZE
                    && nj < SIZE
                    && input.space[ni][nj] >= FIRST_SOME_BYTES
                    && dist[ni][nj].is_none()
                {
                    dist[ni][nj] = dist[i][j].map(|x| x + 1);
                    vd.push_back((ni, nj));
                }
            }
        }
        dist[SIZE - 1][SIZE - 1].unwrap()
    }
    fn part2(input: &Self::Input) -> Self::Answer2 {
        let mut best = vec![vec![0; SIZE]; SIZE];
        let mut bh = BinaryHeap::from_iter([(usize::MAX, (0_usize, 0_usize))]);
        while let Some((t, (i, j))) = bh.pop() {
            for (di, dj) in [0, 1, 0, !0, 0].iter().tuple_windows() {
                let (ni, nj) = (i.wrapping_add(*di), j.wrapping_add(*dj));
                if ni < SIZE && nj < SIZE {
                    let nt = t.min(input.space[ni][nj]);
                    if nt > best[ni][nj] {
                        best[ni][nj] = nt;
                        bh.push((nt, (ni, nj)));
                    }
                }
            }
        }
        let (x, y) = input.bytes[best[SIZE - 1][SIZE - 1]];
        format!("{x},{y}")
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
