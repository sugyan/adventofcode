use aoc2024::{run, Solve};
use itertools::Itertools;
use std::{
    collections::{BTreeSet, HashMap, VecDeque},
    io::{BufRead, BufReader, Read},
};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Hash, PartialEq, Eq)]
enum Side {
    Row(usize, usize),
    Col(usize, usize),
}

struct Solution {
    garden_plots: Vec<Vec<u8>>,
}

impl Solution {
    fn total_price(&self, bulk_discount: bool) -> usize {
        let (rows, cols) = (self.garden_plots.len(), self.garden_plots[0].len());
        let mut seen = vec![vec![false; cols]; rows];
        let mut sum = 0;
        for (i, j) in (0..rows).cartesian_product(0..cols) {
            if seen[i][j] {
                continue;
            }
            let (area, sides) = self.bfs((i, j), &mut seen);
            sum += area
                * sides
                    .values()
                    .map(|v| {
                        if bulk_discount {
                            v.iter()
                                .tuple_windows()
                                .filter(|&(a, b)| *a + 1 < *b)
                                .count()
                                + 1
                        } else {
                            v.len()
                        }
                    })
                    .sum::<usize>();
        }
        sum
    }
    fn bfs(
        &self,
        (i, j): (usize, usize),
        seen: &mut [Vec<bool>],
    ) -> (usize, HashMap<Side, BTreeSet<usize>>) {
        let (rows, cols) = (self.garden_plots.len(), self.garden_plots[0].len());
        let (mut area, mut sides) = (0, HashMap::new());
        let mut vd = [(i, j)].into_iter().collect::<VecDeque<_>>();
        while let Some((i0, j0)) = vd.pop_front() {
            if seen[i0][j0] {
                continue;
            }
            seen[i0][j0] = true;
            area += 1;
            for &(i1, j1) in &[
                (i0.wrapping_sub(1), j0),
                (i0.wrapping_add(1), j0),
                (i0, j0.wrapping_sub(1)),
                (i0, j0.wrapping_add(1)),
            ] {
                if (0..rows).contains(&i1)
                    && (0..cols).contains(&j1)
                    && self.garden_plots[i1][j1] == self.garden_plots[i0][j0]
                {
                    vd.push_back((i1, j1));
                } else {
                    let (key, value) = if i0 != i1 {
                        (Side::Row(i0, i1), j0)
                    } else {
                        (Side::Col(j0, j1), i0)
                    };
                    sides.entry(key).or_insert_with(BTreeSet::new).insert(value);
                }
            }
        }
        (area, sides)
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
        Ok(Self {
            garden_plots: BufReader::new(r)
                .lines()
                .map(|line| line.map(|s| s.bytes().collect()))
                .collect::<Result<_, _>>()?,
        })
    }
    fn part1(&self) -> Self::Answer1 {
        self.total_price(false)
    }
    fn part2(&self) -> Self::Answer2 {
        self.total_price(true)
    }
}

fn main() -> Result<(), Error> {
    run::<Solution>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input_1() -> &'static [u8] {
        r"
AAAA
BBCD
BBCC
EEEC
"[1..]
            .as_bytes()
    }

    fn example_input_2() -> &'static [u8] {
        r"
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
"[1..]
            .as_bytes()
    }

    fn example_input_3() -> &'static [u8] {
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
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input_1())?.part1(), 140);
        assert_eq!(Solution::new(example_input_2())?.part1(), 772);
        assert_eq!(Solution::new(example_input_3())?.part1(), 1930);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input_1())?.part2(), 80);
        assert_eq!(Solution::new(example_input_2())?.part2(), 436);
        assert_eq!(Solution::new(example_input_3())?.part2(), 1206);
        assert_eq!(
            Solution::new(
                r"
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
"[1..]
                    .as_bytes()
            )?
            .part2(),
            236
        );
        assert_eq!(
            Solution::new(
                r"
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
"[1..]
                    .as_bytes()
            )?
            .part2(),
            368
        );
        Ok(())
    }
}
