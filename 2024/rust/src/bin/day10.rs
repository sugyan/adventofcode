use aoc2024::{Solve, run};
use std::{
    collections::HashSet,
    io::{BufRead, BufReader, Read},
};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

trait Evaluate {
    fn unit(_: (usize, usize)) -> Self;
    fn value(&self) -> usize;
    fn accumulate(self, rhs: Self) -> Self;
}

struct Score(HashSet<(usize, usize)>);

impl Evaluate for Score {
    fn unit(p: (usize, usize)) -> Self {
        Self([p].into_iter().collect())
    }
    fn value(&self) -> usize {
        self.0.len()
    }
    fn accumulate(self, rhs: Self) -> Self {
        Self(self.0.union(&rhs.0).copied().collect())
    }
}

struct Rating(usize);

impl Evaluate for Rating {
    fn unit(_: (usize, usize)) -> Self {
        Self(1)
    }
    fn value(&self) -> usize {
        self.0
    }
    fn accumulate(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

struct Solution {
    positions: Vec<HashSet<(usize, usize)>>,
}

impl Solution {
    fn dfs<E>(&self, (i, j): (usize, usize), height: usize) -> Option<E>
    where
        E: Evaluate,
    {
        if height == 9 {
            return Some(E::unit((i, j)));
        }
        [
            (i.wrapping_sub(1), j),
            (i.wrapping_add(1), j),
            (i, j.wrapping_sub(1)),
            (i, j.wrapping_add(1)),
        ]
        .iter()
        .filter(|p| self.positions[height + 1].contains(p))
        .filter_map(|p| self.dfs(*p, height + 1))
        .reduce(E::accumulate)
    }
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;
    type Error = Error;

    fn new<R>(r: R) -> Result<Self, Error>
    where
        R: Read,
    {
        let topographic_map = BufReader::new(r)
            .lines()
            .map(|line| {
                line.map_err(Error::Io)
                    .map(|line| line.bytes().map(|u| (u - b'0').into()).collect())
            })
            .collect::<Result<Vec<Vec<usize>>, _>>()?;
        let mut positions = vec![HashSet::new(); 10];
        for (i, row) in topographic_map.iter().enumerate() {
            for (j, height) in row.iter().enumerate() {
                positions[*height].insert((i, j));
            }
        }
        Ok(Self { positions })
    }
    fn part1(&self) -> Self::Answer1 {
        self.positions[0]
            .iter()
            .filter_map(|p| self.dfs::<Score>(*p, 0).as_ref().map(Evaluate::value))
            .sum()
    }
    fn part2(&self) -> Self::Answer2 {
        self.positions[0]
            .iter()
            .filter_map(|p| self.dfs::<Rating>(*p, 0).as_ref().map(Evaluate::value))
            .sum()
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
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"
        .as_bytes()[1..]
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part1(), 36);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part2(), 81);
        Ok(())
    }
}
