use aoc2024::{run, Solve};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn next(&self, (i, j): (usize, usize)) -> Vec<((usize, usize), Direction)> {
        match self {
            Self::North => vec![
                ((i - 1, j), Self::North),
                ((i, j), Self::West),
                ((i, j), Self::East),
            ],
            Self::South => vec![
                ((i + 1, j), Self::South),
                ((i, j), Self::East),
                ((i, j), Self::West),
            ],
            Self::West => vec![
                ((i, j - 1), Self::West),
                ((i, j), Self::South),
                ((i, j), Self::North),
            ],
            Self::East => vec![
                ((i, j + 1), Self::East),
                ((i, j), Self::North),
                ((i, j), Self::South),
            ],
        }
    }
}

struct Solution {
    maze: Vec<Vec<bool>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;
    type Error = Error;

    fn new<R>(r: R) -> Result<Self, Error>
    where
        R: Read,
    {
        let lines = BufReader::new(r).lines().collect::<Result<Vec<_>, _>>()?;
        let pos = |target| {
            lines
                .iter()
                .enumerate()
                .find_map(|(i, line)| line.chars().position(|c| c == target).map(|j| (i, j)))
        };
        let start = pos('S').ok_or(Error::InvalidInput)?;
        let end = pos('E').ok_or(Error::InvalidInput)?;
        Ok(Self {
            maze: lines
                .into_iter()
                .map(|line| line.chars().map(|c| c != '#').collect())
                .collect(),
            start,
            end,
        })
    }
    fn part1(&self) -> Self::Answer1 {
        let mut min = HashMap::new();
        let mut bh = BinaryHeap::new();
        bh.push((Reverse(0), (self.start, Direction::East)));
        while let Some((Reverse(point), ((i, j), dir))) = bh.pop() {
            if (i, j) == self.end {
                return point;
            }
            if min.contains_key(&(i, j, dir)) {
                continue;
            }
            min.insert((i, j, dir), point);

            for ((i, j), next_dir) in dir.next((i, j)) {
                if self.maze[i][j] {
                    let point = point + if next_dir == dir { 1 } else { 1000 };
                    bh.push((Reverse(point), ((i, j), next_dir)));
                }
            }
        }
        unreachable!()
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
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
"[1..]
            .as_bytes()
    }

    fn example_input_second() -> &'static [u8] {
        r"
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part1(), 7036);
        assert_eq!(Solution::new(example_input_second())?.part1(), 11048);
        Ok(())
    }
}
