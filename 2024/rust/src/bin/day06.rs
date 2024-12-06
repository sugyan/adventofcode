use aoc2024::{run, Solve};
use std::{
    collections::HashSet,
    io::{BufRead, BufReader, Read},
};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("invalid input")]
    InvalidInput,
}

struct Solution {
    map: Vec<Vec<bool>>,
    guard: (usize, usize),
}

impl Solution {
    const DIRECTIONS: [(usize, usize); 4] = [(!0, 0), (0, 1), (1, 0), (0, !0)];
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;
    type Error = Error;

    fn new<R>(r: R) -> Result<Self, Error>
    where
        R: Read,
    {
        let lines = BufReader::new(r).lines().collect::<Result<Vec<_>, _>>()?;
        let guard = lines
            .iter()
            .enumerate()
            .find_map(|(i, line)| line.bytes().position(|u| u == b'^').map(|j| (i, j)))
            .ok_or(Error::InvalidInput)?;
        Ok(Self {
            map: lines
                .iter()
                .map(|line| line.bytes().map(|u| u == b'#').collect())
                .collect(),
            guard,
        })
    }
    fn part1(&self) -> Self::Answer1 {
        let (rows, cols) = (self.map.len(), self.map[0].len());
        let ((mut i, mut j), mut dir_iter) = (self.guard, Self::DIRECTIONS.iter().cycle());
        let (mut di, mut dj) = dir_iter.next().unwrap();
        let mut hs = HashSet::new();
        while (0..rows).contains(&i) && (0..cols).contains(&j) {
            hs.insert((i, j));
            let (ii, jj) = (i.wrapping_add(di), j.wrapping_add(dj));
            if (0..rows).contains(&ii) && (0..cols).contains(&jj) && self.map[ii][jj] {
                (di, dj) = *dir_iter.next().unwrap();
            } else {
                (i, j) = (ii, jj);
            }
        }
        hs.len()
    }
    fn part2(&self) -> Self::Answer2 {
        todo!()
    }
}

fn main() -> Result<(), Error> {
    run::<Solution>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static [u8] {
        r"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part1(), 41);
        Ok(())
    }
}
