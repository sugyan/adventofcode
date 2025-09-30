use aoc2024::{Day, run_day};
use std::{collections::HashSet, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {}

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

struct Input(Vec<HashSet<(usize, usize)>>);

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let topographic_map = s
            .lines()
            .map(|line| line.bytes().map(|u| (u - b'0').into()).collect())
            .collect::<Vec<Vec<usize>>>();
        let mut positions = vec![HashSet::new(); 10];
        for (i, row) in topographic_map.iter().enumerate() {
            for (j, height) in row.iter().enumerate() {
                positions[*height].insert((i, j));
            }
        }
        Ok(Self(positions))
    }
}

struct Solution;

impl Solution {
    fn evaluate_trailhead<E>(input: &Input) -> usize
    where
        E: Evaluate,
    {
        input.0[0]
            .iter()
            .filter_map(|p| Self::dfs(input, *p, 0).as_ref().map(E::value))
            .sum()
    }
    fn dfs<E>(input: &Input, (i, j): (usize, usize), height: usize) -> Option<E>
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
        .filter(|p| input.0[height + 1].contains(p))
        .filter_map(|p| Self::dfs(input, *p, height + 1))
        .reduce(E::accumulate)
    }
}

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = usize;
    type Answer2 = usize;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        Self::evaluate_trailhead::<Score>(input)
    }
    fn part2(input: &Self::Input) -> Self::Answer2 {
        Self::evaluate_trailhead::<Rating>(input)
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
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"
        .trim_start()
        .parse()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::part1(&example_input()?), 36);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::part2(&example_input()?), 81);
        Ok(())
    }
}
