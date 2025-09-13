use aoc2024::{Solve, run};
use itertools::Itertools;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    iter,
};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

struct Solution {
    map_size: (usize, usize),
    antennas: HashMap<u8, Vec<(usize, usize)>>,
}

impl Solution {
    fn count_antinodes(&self, update: bool) -> usize {
        self.antennas
            .values()
            .flat_map(|v| {
                v.iter().combinations(2).flat_map(|c| {
                    let (di, dj) = (c[1].0.wrapping_sub(c[0].0), c[1].1.wrapping_sub(c[0].1));
                    let it0 = iter::successors(Some(*c[0]), move |(i, j)| {
                        Some((i.wrapping_sub(di), j.wrapping_sub(dj))).filter(|(i, j)| {
                            (0..self.map_size.0).contains(i) && (0..self.map_size.1).contains(j)
                        })
                    });
                    let it1 = iter::successors(Some(*c[1]), move |(i, j)| {
                        Some((i.wrapping_add(di), j.wrapping_add(dj))).filter(|(i, j)| {
                            (0..self.map_size.0).contains(i) && (0..self.map_size.1).contains(j)
                        })
                    });
                    if update {
                        it0.chain(it1).collect_vec()
                    } else {
                        it0.skip(1).take(1).chain(it1.skip(1).take(1)).collect_vec()
                    }
                })
            })
            .unique()
            .count()
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
        let map = BufReader::new(r)
            .lines()
            .map(|line| line.map(|line| line.bytes().collect()))
            .collect::<Result<Vec<Vec<_>>, _>>()?;
        let mut antennas = HashMap::new();
        for (i, row) in map.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if col != &b'.' {
                    antennas.entry(*col).or_insert_with(Vec::new).push((i, j));
                }
            }
        }
        Ok(Self {
            map_size: (map.len(), map[0].len()),
            antennas,
        })
    }
    fn part1(&self) -> Self::Answer1 {
        self.count_antinodes(false)
    }
    fn part2(&self) -> Self::Answer2 {
        self.count_antinodes(true)
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
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part1(), 14);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part2(), 34);
        Ok(())
    }
}
