use aoc2024::{run, Solve};
use std::{
    collections::{HashMap, HashSet, VecDeque},
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
    racetrack: HashSet<(usize, usize)>,
    start: (usize, usize),
    #[allow(dead_code)]
    end: (usize, usize),
}

impl Solution {
    fn cheat_counts(&self, threashold: usize) -> usize {
        let mut mins = [(self.start, 0)].into_iter().collect::<HashMap<_, _>>();
        let mut vd = [(self.start, 0)].into_iter().collect::<VecDeque<_>>();
        while let Some((p, d)) = vd.pop_front() {
            mins.insert(p, d);
            for (di, dj) in [(!0, 0), (1, 0), (0, !0), (0, 1)] {
                let (i, j) = (p.0.wrapping_add(di), p.1.wrapping_add(dj));
                if self.racetrack.contains(&(i, j)) && !mins.contains_key(&(i, j)) {
                    vd.push_back(((i, j), d + 1));
                }
            }
        }
        let mut groups = HashMap::new();
        for (&(i, j), min_src) in &mins {
            for p in &[
                (i.wrapping_sub(2), j),
                (i.wrapping_sub(1), j.wrapping_sub(1)),
                (i, j.wrapping_sub(2)),
                (i.wrapping_add(1), j.wrapping_sub(1)),
                (i.wrapping_add(2), j),
                (i.wrapping_add(1), j.wrapping_add(1)),
                (i, j.wrapping_add(2)),
                (i.wrapping_sub(1), j.wrapping_add(1)),
            ] {
                if let Some(min_dst) = mins.get(p) {
                    if min_src + 2 < *min_dst {
                        *groups.entry(min_dst - min_src - 2).or_insert(0) += 1;
                    }
                }
            }
        }
        groups
            .iter()
            .filter(|(k, _)| **k >= threashold)
            .map(|(_, v)| v)
            .sum()
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
        let lines = BufReader::new(r).lines().collect::<Result<Vec<_>, _>>()?;
        let mut racetrack = HashSet::new();
        for (i, row) in lines.iter().enumerate() {
            for (j, u) in row.bytes().enumerate() {
                if u != b'#' {
                    racetrack.insert((i, j));
                }
            }
        }
        let find = |target| {
            lines.iter().enumerate().find_map(|(i, row)| {
                row.bytes()
                    .enumerate()
                    .find(|(_, u)| *u == target)
                    .map(|(j, _)| (i, j))
            })
        };
        Ok(Self {
            racetrack,
            start: find(b'S').ok_or(Error::InvalidInput)?,
            end: find(b'E').ok_or(Error::InvalidInput)?,
        })
    }
    fn part1(&self) -> Self::Answer1 {
        self.cheat_counts(100)
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
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
"[1..]
            .as_bytes()
    }

    #[test]
    fn cheat_counts() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.cheat_counts(64), 1);
        assert_eq!(Solution::new(example_input())?.cheat_counts(40), 2);
        assert_eq!(Solution::new(example_input())?.cheat_counts(38), 3);
        assert_eq!(Solution::new(example_input())?.cheat_counts(36), 4);
        assert_eq!(Solution::new(example_input())?.cheat_counts(20), 5);
        assert_eq!(Solution::new(example_input())?.cheat_counts(12), 8);
        assert_eq!(Solution::new(example_input())?.cheat_counts(10), 10);
        assert_eq!(Solution::new(example_input())?.cheat_counts(8), 14);
        assert_eq!(Solution::new(example_input())?.cheat_counts(6), 16);
        assert_eq!(Solution::new(example_input())?.cheat_counts(4), 30);
        assert_eq!(Solution::new(example_input())?.cheat_counts(2), 44);
        Ok(())
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part1(), 0);
        Ok(())
    }
}
