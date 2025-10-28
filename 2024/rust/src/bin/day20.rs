use aoc2024::{Day, run_day};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("invalid input")]
    InvalidInput,
}

struct Input {
    racetrack: HashSet<(isize, isize)>,
    start: (isize, isize),
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();
        let mut racetrack = HashSet::new();
        let mut start = None;
        for (i, row) in (0..).zip(&lines) {
            for (j, u) in (0..).zip(row.bytes()) {
                if u == b'S' {
                    start = Some((i, j));
                }
                if u != b'#' {
                    racetrack.insert((i, j));
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
        let mins = Self::bfs(input);
        let mut sum = 0;
        for (&(i, j), min_src) in &mins {
            sum += (-seconds..=seconds)
                .flat_map(|di| {
                    let r = seconds - di.abs();
                    (-r..=r).map(move |dj| (di, dj))
                })
                .filter(|(di, dj)| {
                    mins.get(&(i + di, j + dj))
                        .map(|min_dst| min_src + di.abs() + dj.abs() + threshold <= *min_dst)
                        .unwrap_or_default()
                })
                .count();
        }
        sum
    }
    fn bfs(input: &Input) -> HashMap<(isize, isize), isize> {
        let mut mins = [(input.start, 0)].into_iter().collect::<HashMap<_, _>>();
        let mut vd = [(input.start, 0)].into_iter().collect::<VecDeque<_>>();
        while let Some((p, d)) = vd.pop_front() {
            mins.insert(p, d);
            for (di, dj) in [(!0, 0), (1, 0), (0, !0), (0, 1)] {
                let (i, j) = (p.0.wrapping_add(di), p.1.wrapping_add(dj));
                if input.racetrack.contains(&(i, j)) && !mins.contains_key(&(i, j)) {
                    vd.push_back(((i, j), d + 1));
                }
            }
        }
        mins
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
