use aoc2024::{Day, run_day};
use itertools::Itertools;
use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("invalid input")]
    InvalidInput,
}

struct Input {
    racetrack: Vec<Vec<bool>>,
    start: (usize, usize),
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();
        let mut racetrack = vec![vec![false; lines[0].len()]; lines.len()];
        let mut start = None;
        for (i, row) in lines.iter().enumerate() {
            for (j, c) in row.chars().enumerate() {
                if c != '#' {
                    racetrack[i][j] = true;
                }
                if c == 'S' {
                    start = Some((i, j));
                }
            }
        }
        Ok(Self {
            racetrack,
            start: start.ok_or(Error::InvalidInput)?,
        })
    }
}

struct Solution;

impl Solution {
    fn cheat_counts(input: &Input, seconds: isize, threshold: isize) -> usize {
        let dist = Self::bfs(input);
        let dist_map = (0..)
            .zip(dist.iter())
            .flat_map(|(i, row)| {
                (0..)
                    .zip(row.iter())
                    .filter_map(move |(j, &o)| o.map(|d| ((i, j), d)))
            })
            .collect::<HashMap<(isize, isize), _>>();
        let offsets = (-seconds..=seconds)
            .flat_map(|di| {
                let r = seconds - di.abs();
                (-r..=r).map(move |dj| ((di, dj), di.abs() + dj.abs()))
            })
            .collect_vec();
        dist_map
            .iter()
            .map(|((i, j), src)| {
                offsets
                    .iter()
                    .filter(|((di, dj), d)| {
                        dist_map
                            .get(&(i + di, j + dj))
                            .is_some_and(|&dst| dst - d - src >= threshold)
                    })
                    .count()
            })
            .sum()
    }
    fn bfs(input: &Input) -> Vec<Vec<Option<isize>>> {
        let mut dist = vec![vec![None; input.racetrack[0].len()]; input.racetrack.len()];
        let mut vd = [(input.start, 0)].into_iter().collect::<VecDeque<_>>();
        while let Some(((i, j), d)) = vd.pop_front() {
            dist[i][j] = Some(d);
            for (ni, nj) in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
                if input.racetrack[ni][nj] && dist[ni][nj].is_none() {
                    vd.push_back(((ni, nj), d + 1));
                }
            }
        }
        dist
    }
}

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = usize;
    type Answer2 = usize;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        Self::cheat_counts(input, 2, 100)
    }
    fn part2(input: &Self::Input) -> Self::Answer2 {
        Self::cheat_counts(input, 20, 100)
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
"
        .trim_start()
        .parse()
    }

    #[test]
    fn cheat_counts() -> Result<(), Error> {
        // 2
        assert_eq!(Solution::cheat_counts(&example_input()?, 2, 64), 1);
        assert_eq!(Solution::cheat_counts(&example_input()?, 2, 40), 2);
        assert_eq!(Solution::cheat_counts(&example_input()?, 2, 38), 3);
        assert_eq!(Solution::cheat_counts(&example_input()?, 2, 36), 4);
        assert_eq!(Solution::cheat_counts(&example_input()?, 2, 20), 5);
        assert_eq!(Solution::cheat_counts(&example_input()?, 2, 12), 8);
        assert_eq!(Solution::cheat_counts(&example_input()?, 2, 10), 10);
        assert_eq!(Solution::cheat_counts(&example_input()?, 2, 8), 14);
        assert_eq!(Solution::cheat_counts(&example_input()?, 2, 6), 16);
        assert_eq!(Solution::cheat_counts(&example_input()?, 2, 4), 30);
        assert_eq!(Solution::cheat_counts(&example_input()?, 2, 2), 44);
        // 20
        assert_eq!(Solution::cheat_counts(&example_input()?, 20, 76), 3);
        assert_eq!(Solution::cheat_counts(&example_input()?, 20, 74), 7);
        assert_eq!(Solution::cheat_counts(&example_input()?, 20, 72), 29);
        assert_eq!(Solution::cheat_counts(&example_input()?, 20, 70), 41);
        assert_eq!(Solution::cheat_counts(&example_input()?, 20, 68), 55);
        assert_eq!(Solution::cheat_counts(&example_input()?, 20, 66), 67);
        assert_eq!(Solution::cheat_counts(&example_input()?, 20, 64), 86);
        assert_eq!(Solution::cheat_counts(&example_input()?, 20, 62), 106);
        assert_eq!(Solution::cheat_counts(&example_input()?, 20, 60), 129);
        assert_eq!(Solution::cheat_counts(&example_input()?, 20, 58), 154);
        assert_eq!(Solution::cheat_counts(&example_input()?, 20, 56), 193);
        assert_eq!(Solution::cheat_counts(&example_input()?, 20, 54), 222);
        assert_eq!(Solution::cheat_counts(&example_input()?, 20, 52), 253);
        assert_eq!(Solution::cheat_counts(&example_input()?, 20, 50), 285);
        Ok(())
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::part1(&example_input()?), 0);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::part2(&example_input()?), 0);
        Ok(())
    }
}
