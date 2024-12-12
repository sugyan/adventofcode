use aoc2024::{run, Solve};
use std::{
    collections::VecDeque,
    io::{BufRead, BufReader, Read},
};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

struct Solution {
    garden_plots: Vec<Vec<u8>>,
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;
    type Error = Error;

    fn new<R>(r: R) -> Result<Self, Error>
    where
        R: Read,
    {
        Ok(Self {
            garden_plots: BufReader::new(r)
                .lines()
                .map(|line| line.map(|s| s.bytes().collect()))
                .collect::<Result<_, _>>()?,
        })
    }
    fn part1(&self) -> Self::Answer1 {
        let (rows, cols) = (self.garden_plots.len(), self.garden_plots[0].len());
        let mut seen = vec![vec![false; cols]; rows];
        let mut sum = 0;
        for i in 0..rows {
            for j in 0..cols {
                if !seen[i][j] {
                    let u = self.garden_plots[i][j];
                    let (mut area, mut perimeter) = (0, 0);
                    let mut vd = [(i, j)].into_iter().collect::<VecDeque<_>>();
                    while let Some((i, j)) = vd.pop_front() {
                        if seen[i][j] {
                            continue;
                        }
                        seen[i][j] = true;
                        area += 1;
                        perimeter += 4;
                        for &(i, j) in [
                            (i.wrapping_sub(1), j),
                            (i.wrapping_add(1), j),
                            (i, j.wrapping_sub(1)),
                            (i, j.wrapping_add(1)),
                        ]
                        .iter()
                        .filter(|(i, j)| {
                            (0..rows).contains(i)
                                && (0..cols).contains(j)
                                && self.garden_plots[*i][*j] == u
                        }) {
                            if seen[i][j] {
                                perimeter -= 2;
                            } else {
                                vd.push_back((i, j));
                            }
                        }
                    }
                    sum += area * perimeter;
                }
            }
        }
        sum
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

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(
            Solution::new(
                r"
AAAA
BBCD
BBCC
EEEC
"[1..]
                    .as_bytes()
            )?
            .part1(),
            140
        );
        assert_eq!(
            Solution::new(
                r"
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
"[1..]
                    .as_bytes()
            )?
            .part1(),
            772
        );
        assert_eq!(
            Solution::new(
                r"
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"[1..]
                    .as_bytes()
            )?
            .part1(),
            1930
        );
        Ok(())
    }
}
