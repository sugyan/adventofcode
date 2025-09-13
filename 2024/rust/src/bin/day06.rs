use aoc2024::{Solve, run};
use std::{
    collections::{HashMap, HashSet, hash_map::Entry},
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
    fn next_position(&self, (i, j): (usize, usize)) -> (usize, usize) {
        match self {
            Self::Up => (i.wrapping_sub(1), j),
            Self::Right => (i, j.wrapping_add(1)),
            Self::Down => (i.wrapping_add(1), j),
            Self::Left => (i, j.wrapping_sub(1)),
        }
    }
}

struct Solution {
    map: Vec<Vec<bool>>,
    guard: (usize, usize),
}

impl Solution {
    fn simulate_patrol(
        &self,
        map: &[Vec<bool>],
    ) -> Option<HashMap<(usize, usize), HashSet<Direction>>> {
        let (rows, cols) = (map.len(), map[0].len());
        let ((mut i, mut j), mut dir) = (self.guard, Direction::Up);
        let mut hm = HashMap::<(usize, usize), HashSet<Direction>>::new();
        while (0..rows).contains(&i) && (0..cols).contains(&j) {
            match hm.entry((i, j)) {
                Entry::Occupied(mut oe) => {
                    if oe.get().contains(&dir) {
                        return None;
                    }
                    oe.get_mut().insert(dir);
                }
                Entry::Vacant(ve) => {
                    ve.insert(HashSet::new());
                }
            }
            let (ii, jj) = dir.next_position((i, j));
            if (0..rows).contains(&ii) && (0..cols).contains(&jj) && map[ii][jj] {
                dir = dir.turn_right();
            } else {
                (i, j) = (ii, jj);
            }
        }
        Some(hm)
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
        let guard = lines
            .iter()
            .enumerate()
            .find_map(|(i, line)| line.bytes().position(|u| u == b'^').map(|j| (i, j)))
            .ok_or(Error::InvalidInput)?;
        Ok(Self {
            map: lines
                .iter()
                .map(|line| line.bytes().map(|u| u == b'#').collect())
                .collect(),
            guard,
        })
    }
    fn part1(&self) -> Self::Answer1 {
        self.simulate_patrol(&self.map).unwrap().len()
    }
    fn part2(&self) -> Self::Answer2 {
        self.simulate_patrol(&self.map)
            .unwrap()
            .keys()
            .filter(|(i, j)| {
                if (*i, *j) == self.guard {
                    return false;
                }
                let mut map = self.map.clone();
                map[*i][*j] = true;
                self.simulate_patrol(&map).is_none()
            })
            .count()
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
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part1(), 41);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part2(), 6);
        Ok(())
    }
}
