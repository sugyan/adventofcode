use aoc2024::{run, Solve};
use itertools::Itertools;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

struct Solution {
    map: Vec<Vec<u8>>,
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;
    type Error = Error;

    fn new<R>(r: R) -> Result<Self, Error>
    where
        R: Read,
    {
        Ok(Self {
            map: BufReader::new(r)
                .lines()
                .map(|line| line.map(|line| line.bytes().collect()))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
    fn part1(&self) -> Self::Answer1 {
        let (row, col) = (self.map.len(), self.map[0].len());
        let mut hm = HashMap::new();
        for (i, row) in self.map.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if col != &b'.' {
                    hm.entry(col).or_insert_with(Vec::new).push((i, j));
                }
            }
        }
        hm.values()
            .flat_map(|v| {
                v.iter().combinations(2).flat_map(|c| {
                    let (di, dj) = (c[0].0.wrapping_sub(c[1].0), c[0].1.wrapping_sub(c[1].1));
                    vec![
                        (c[0].0.wrapping_add(di), c[0].1.wrapping_add(dj)),
                        (c[1].0.wrapping_sub(di), c[1].1.wrapping_sub(dj)),
                    ]
                })
            })
            .unique()
            .filter(|(i, j)| (0..row).contains(i) && (0..col).contains(j))
            .count()
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
}
