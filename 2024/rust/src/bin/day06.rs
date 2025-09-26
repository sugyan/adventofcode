use aoc2024::{Day, run_day};
use itertools::Itertools;
use std::{collections::HashSet, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("invalid input")]
    InvalidInput,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

type Position = ((usize, usize), Direction);

#[derive(Clone)]
struct Area {
    map: Vec<Vec<bool>>,
    rows: usize,
    cols: usize,
}

impl Area {
    fn distinct_positions(&self, start: Position) -> HashSet<(usize, usize)> {
        let mut path = HashSet::from_iter([start.0]);
        let mut p = start;
        while let Some(next) = self.next_position(p) {
            path.insert(next.0);
            p = next;
        }
        path
    }
    fn will_stuck_in_loop(&self, start: Position) -> bool {
        let mut seen = HashSet::new();
        let mut p = start;
        while let Some(next) = self.next_position(p) {
            if next.1 != p.1 && !seen.insert(next) {
                return true;
            }
            p = next;
        }
        false
    }
    fn next_position(&self, ((i, j), d): Position) -> Option<Position> {
        let (next_i, next_j) = match d {
            Direction::Up => (i.checked_sub(1)?, j),
            Direction::Right => (i, j + 1),
            Direction::Down => (i + 1, j),
            Direction::Left => (i, j.checked_sub(1)?),
        };
        if next_i < self.rows && next_j < self.cols {
            Some(if self.map[next_i][next_j] {
                ((i, j), d.turn_right())
            } else {
                ((next_i, next_j), d)
            })
        } else {
            None
        }
    }
}

impl FromStr for Area {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .lines()
            .map(|line| line.bytes().map(|u| u == b'#').collect_vec())
            .collect_vec();
        let (rows, cols) = (map.len(), map[0].len());
        Ok(Self { map, rows, cols })
    }
}

struct Input {
    area: Area,
    start: ((usize, usize), Direction),
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();
        let guard_pos = lines
            .iter()
            .enumerate()
            .find_map(|(i, line)| line.bytes().position(|u| u == b'^').map(|j| (i, j)))
            .ok_or(Error::InvalidInput)?;
        Ok(Self {
            area: s.parse()?,
            start: (guard_pos, Direction::Up),
        })
    }
}

struct Solution;

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = usize;
    type Answer2 = usize;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        input.area.distinct_positions(input.start).len()
    }
    fn part2(input: &Self::Input) -> Self::Answer2 {
        let mut area = input.area.clone();
        input
            .area
            .distinct_positions(input.start)
            .iter()
            .filter(|(i, j)| {
                area.map[*i][*j] = true;
                let result = area.will_stuck_in_loop(input.start);
                area.map[*i][*j] = false;
                result
            })
            .count()
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
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"
        .trim_start()
        .parse()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::part1(&example_input()?), 41);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::part2(&example_input()?), 6);
        Ok(())
    }
}
