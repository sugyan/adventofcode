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
    racetrack: HashSet<(isize, isize)>,
    start: (isize, isize),
}

impl Solution {
    fn cheat_counts(&self, seconds: isize, threashold: isize) -> usize {
        let mins = self.bfs();
        let mut sum = 0;
        for (&(i, j), min_src) in &mins {
            sum += (-seconds..=seconds)
                .flat_map(|di| {
                    let r = seconds - di.abs();
                    (-r..=r).map(move |dj| (di, dj))
                })
                .filter(|(di, dj)| {
                    mins.get(&(i + di, j + dj))
                        .map(|min_dst| min_src + di.abs() + dj.abs() + threashold <= *min_dst)
                        .unwrap_or_default()
                })
                .count();
        }
        sum
    }
    fn bfs(&self) -> HashMap<(isize, isize), isize> {
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
        mins
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
        for (i, row) in (0..).zip(&lines) {
            for (j, u) in (0..).zip(row.bytes()) {
                if u != b'#' {
                    racetrack.insert((i, j));
                }
            }
        }

        Ok(Self {
            racetrack,
            start: (0..)
                .zip(lines)
                .find_map(|(i, row)| {
                    (0..)
                        .zip(row.bytes())
                        .find(|(_, u)| *u == b'S')
                        .map(|(j, _)| (i, j))
                })
                .ok_or(Error::InvalidInput)?,
        })
    }
    fn part1(&self) -> Self::Answer1 {
        self.cheat_counts(2, 100)
    }
    fn part2(&self) -> Self::Answer2 {
        self.cheat_counts(20, 100)
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
        // 2
        assert_eq!(Solution::new(example_input())?.cheat_counts(2, 64), 1);
        assert_eq!(Solution::new(example_input())?.cheat_counts(2, 40), 2);
        assert_eq!(Solution::new(example_input())?.cheat_counts(2, 38), 3);
        assert_eq!(Solution::new(example_input())?.cheat_counts(2, 36), 4);
        assert_eq!(Solution::new(example_input())?.cheat_counts(2, 20), 5);
        assert_eq!(Solution::new(example_input())?.cheat_counts(2, 12), 8);
        assert_eq!(Solution::new(example_input())?.cheat_counts(2, 10), 10);
        assert_eq!(Solution::new(example_input())?.cheat_counts(2, 8), 14);
        assert_eq!(Solution::new(example_input())?.cheat_counts(2, 6), 16);
        assert_eq!(Solution::new(example_input())?.cheat_counts(2, 4), 30);
        assert_eq!(Solution::new(example_input())?.cheat_counts(2, 2), 44);
        // 20
        assert_eq!(Solution::new(example_input())?.cheat_counts(20, 76), 3);
        assert_eq!(Solution::new(example_input())?.cheat_counts(20, 74), 7);
        assert_eq!(Solution::new(example_input())?.cheat_counts(20, 72), 29);
        assert_eq!(Solution::new(example_input())?.cheat_counts(20, 70), 41);
        assert_eq!(Solution::new(example_input())?.cheat_counts(20, 68), 55);
        assert_eq!(Solution::new(example_input())?.cheat_counts(20, 66), 67);
        assert_eq!(Solution::new(example_input())?.cheat_counts(20, 64), 86);
        assert_eq!(Solution::new(example_input())?.cheat_counts(20, 62), 106);
        assert_eq!(Solution::new(example_input())?.cheat_counts(20, 60), 129);
        assert_eq!(Solution::new(example_input())?.cheat_counts(20, 58), 154);
        assert_eq!(Solution::new(example_input())?.cheat_counts(20, 56), 193);
        assert_eq!(Solution::new(example_input())?.cheat_counts(20, 54), 222);
        assert_eq!(Solution::new(example_input())?.cheat_counts(20, 52), 253);
        assert_eq!(Solution::new(example_input())?.cheat_counts(20, 50), 285);
        Ok(())
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part1(), 0);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part2(), 0);
        Ok(())
    }
}
