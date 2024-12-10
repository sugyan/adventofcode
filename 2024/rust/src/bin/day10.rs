use aoc2024::{run, Solve};
use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader, Read},
};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

struct Solution {
    topographic_map: Vec<Vec<usize>>,
    heights: Vec<HashSet<(usize, usize)>>,
}

impl Solution {
    fn hiking_trails(
        &self,
    ) -> impl Iterator<Item = std::collections::HashMap<(&usize, &usize), usize>> {
        let (rows, cols) = (self.topographic_map.len(), self.topographic_map[0].len());
        let mut trails = vec![vec![HashMap::new(); cols]; rows];
        for (i, j) in &self.heights[9] {
            trails[*i][*j].insert((i, j), 1);
        }
        for h in (0..9).rev() {
            for (i, j) in &self.heights[h] {
                trails[*i][*j] = [
                    (i.wrapping_sub(1), *j),
                    (i.wrapping_add(1), *j),
                    (*i, j.wrapping_sub(1)),
                    (*i, j.wrapping_add(1)),
                ]
                .iter()
                .filter(|p| self.heights[h + 1].contains(p))
                .flat_map(|(i, j)| &trails[*i][*j])
                .fold(HashMap::new(), |mut acc, (k, v)| {
                    *acc.entry(*k).or_default() += v;
                    acc
                });
            }
        }
        self.heights[0]
            .iter()
            .map(move |(i, j)| trails[*i][*j].clone())
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
                    .map(|line| line.bytes().map(|u| (u - b'0') as usize).collect())
            })
            .collect::<Result<Vec<Vec<_>>, _>>()?;
        let mut heights = vec![HashSet::new(); 10];
        for (i, row) in topographic_map.iter().enumerate() {
            for (j, &h) in row.iter().enumerate() {
                heights[h].insert((i, j));
            }
        }
        Ok(Self {
            topographic_map,
            heights,
        })
    }
    fn part1(&self) -> Self::Answer1 {
        self.hiking_trails().map(|hm| hm.len()).sum()
    }
    fn part2(&self) -> Self::Answer2 {
        self.hiking_trails()
            .map(|hm| hm.values().sum::<usize>())
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
        r"
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"[1..]
            .as_bytes()
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
