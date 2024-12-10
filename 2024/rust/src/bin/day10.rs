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
}

struct Solution {
    topographic_map: Vec<Vec<u8>>,
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
            topographic_map: BufReader::new(r)
                .lines()
                .map(|line| {
                    line.map_err(Error::Io)
                        .map(|line| line.bytes().map(|u| u - b'0').collect())
                })
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
    fn part1(&self) -> Self::Answer1 {
        let (rows, cols) = (self.topographic_map.len(), self.topographic_map[0].len());
        let mut counts = vec![vec![HashSet::new(); cols]; rows];
        for (i, row) in counts.iter_mut().enumerate() {
            for (j, col) in row.iter_mut().enumerate() {
                if self.topographic_map[i][j] == 9 {
                    col.insert((i, j));
                }
            }
        }
        for h in (0..9).rev() {
            for i in 0..rows {
                for j in 0..cols {
                    if self.topographic_map[i][j] == h {
                        counts[i][j] = [
                            (i.wrapping_sub(1), j),
                            (i.wrapping_add(1), j),
                            (i, j.wrapping_sub(1)),
                            (i, j.wrapping_add(1)),
                        ]
                        .iter()
                        .filter(|(i, j)| {
                            (0..rows).contains(i)
                                && (0..cols).contains(j)
                                && self.topographic_map[*i][*j] == h + 1
                        })
                        .flat_map(|(i, j)| counts[*i][*j].iter().cloned())
                        .collect();
                    }
                }
            }
        }
        counts
            .iter()
            .flatten()
            .zip(self.topographic_map.iter().flatten())
            .filter_map(|(s, h)| if *h == 0 { Some(s.len()) } else { None })
            .sum()
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
}
