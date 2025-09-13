use aoc2024::{Solve, run};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
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

type PosWithDir = ((usize, usize), Direction);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn next_pos(&self, (i, j): (usize, usize)) -> (usize, usize) {
        match self {
            Self::North => (i - 1, j),
            Self::South => (i + 1, j),
            Self::West => (i, j - 1),
            Self::East => (i, j + 1),
        }
    }
    fn next_dirs(&self) -> [Self; 2] {
        match self {
            Self::North => [Self::East, Self::West],
            Self::South => [Self::West, Self::East],
            Self::West => [Self::North, Self::South],
            Self::East => [Self::South, Self::North],
        }
    }
}

struct Solution {
    maze: Vec<Vec<bool>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Solution {
    fn dijkstra(&self) -> HashMap<PosWithDir, Vec<PosWithDir>> {
        let start = (self.start, Direction::East);
        let mut prevs = [(start, Vec::new())].into_iter().collect::<HashMap<_, _>>();
        let mut mins = [(start, 0)].into_iter().collect::<HashMap<_, _>>();
        let mut bh = BinaryHeap::new();
        bh.push((Reverse(0), (self.start, Direction::East)));
        while let Some((Reverse(point), (p, dir))) = bh.pop() {
            if p == self.end {
                break;
            }
            let (i, j) = dir.next_pos(p);
            let nexts = dir
                .next_dirs()
                .into_iter()
                .map(|d| ((p, d), point + 1000))
                .chain(if self.maze[i][j] {
                    vec![(((i, j), dir), point + 1)]
                } else {
                    vec![]
                });
            for (next, point) in nexts {
                if let Some(min) = mins.get_mut(&next) {
                    if point == *min {
                        prevs.entry(next).or_default().push((p, dir));
                    }
                } else {
                    mins.insert(next, point);
                    prevs.insert(next, vec![(p, dir)]);
                    bh.push((Reverse(point), next));
                }
            }
        }
        prevs
    }
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = usize;
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
        let prevs = self.dijkstra();
        let mut points = 0;
        let mut curr = prevs.keys().find(|(p, _)| p == &self.end);
        while let Some(p) = curr {
            let prev = prevs.get(p).and_then(|v| v.first());
            if let Some((_, dir)) = &prev {
                points += if *dir == p.1 { 1 } else { 1000 };
            }
            curr = prev;
        }
        points
    }
    fn part2(&self) -> Self::Answer2 {
        fn dfs(
            p: PosWithDir,
            prevs: &HashMap<PosWithDir, Vec<PosWithDir>>,
            visited: &mut HashSet<(usize, usize)>,
        ) {
            visited.insert(p.0);
            if let Some(nexts) = prevs.get(&p) {
                for next in nexts {
                    dfs(*next, prevs, visited);
                }
            }
        }

        let prevs = self.dijkstra();
        let mut visited = HashSet::new();
        for p in prevs.keys().filter(|(p, _)| p == &self.end) {
            dfs(*p, &prevs, &mut visited);
        }
        visited.len()
    }
}

fn main() -> Result<(), Error> {
    run::<Solution>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static [u8] {
        &r"
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
"
        .as_bytes()[1..]
    }

    fn example_input_second() -> &'static [u8] {
        &r"
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
"
        .as_bytes()[1..]
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part1(), 7036);
        assert_eq!(Solution::new(example_input_second())?.part1(), 11048);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part2(), 45);
        assert_eq!(Solution::new(example_input_second())?.part2(), 64);
        Ok(())
    }
}
