use aoc2024::{run, Solve};
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{BufRead, BufReader, Read},
};
use thiserror::Error;

const ADJACENTS: [(usize, usize); 4] = [(!0, 0), (1, 0), (0, !0), (0, 1)];

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

struct Solution {
    garden_plots: HashMap<(usize, usize), u8>,
}

impl Solution {
    fn total_price(&self, bulk_discount: bool) -> usize {
        let mut sum = 0;
        let mut seen = HashSet::new();
        for (p, u) in &self.garden_plots {
            if !seen.contains(p) {
                let area = self.bfs(p, u, &mut seen);
                sum += area.len()
                    * if bulk_discount {
                        self.count_corners(&area, u)
                    } else {
                        self.calculate_perimeter(&area, u)
                    };
            }
        }
        sum
    }
    fn bfs(
        &self,
        p: &(usize, usize),
        u: &u8,
        seen: &mut HashSet<(usize, usize)>,
    ) -> Vec<(usize, usize)> {
        let mut area = Vec::new();
        let mut vd = [*p].into_iter().collect::<VecDeque<_>>();
        seen.insert(*p);
        while let Some(p) = vd.pop_front() {
            area.push(p);
            for (di, dj) in ADJACENTS {
                let (i, j) = (p.0.wrapping_add(di), p.1.wrapping_add(dj));
                if self.garden_plots.get(&(i, j)) == Some(u) && !seen.contains(&(i, j)) {
                    seen.insert((i, j));
                    vd.push_back((i, j));
                }
            }
        }
        area
    }
    fn calculate_perimeter(&self, area: &[(usize, usize)], u: &u8) -> usize {
        area.iter()
            .cartesian_product(ADJACENTS)
            .filter(|&((i, j), (di, dj))| {
                self.garden_plots
                    .get(&(i.wrapping_add(di), j.wrapping_add(dj)))
                    != Some(u)
            })
            .count()
    }
    fn count_corners(&self, area: &[(usize, usize)], u: &u8) -> usize {
        area.iter()
            .flat_map(|(i, j)| {
                [
                    (!0, 0),
                    (!0, 1),
                    (0, 1),
                    (1, 1),
                    (1, 0),
                    (1, !0),
                    (0, !0),
                    (!0, !0),
                ]
                .iter()
                .map(|&(di, dj)| {
                    self.garden_plots
                        .get(&(i.wrapping_add(di), j.wrapping_add(dj)))
                        != Some(u)
                })
                .circular_tuple_windows()
                .step_by(2)
            })
            .filter(|t| matches!(t, (true, _, true) | (false, true, false)))
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
            .map(|line| line.map(|s| s.bytes().collect()))
            .collect::<Result<Vec<Vec<u8>>, _>>()?;
        Ok(Self {
            garden_plots: map
                .iter()
                .enumerate()
                .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, u)| ((i, j), *u)))
                .collect(),
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
