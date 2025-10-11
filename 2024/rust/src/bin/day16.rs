use aoc2024::{Day, run_day};
use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque, hash_map::Entry},
    str::FromStr,
};
use thiserror::Error;

const STEP: u32 = 1;
const TURN: u32 = 1000;

#[derive(Error, Debug)]
enum Error {
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

struct Input {
    maze: Vec<Vec<bool>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();
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
}

struct Solution;

impl Solution {
    fn dijkstra_with_paths(input: &Input) -> HashMap<PosWithDir, (u32, Vec<PosWithDir>)> {
        let start = (input.start, Direction::East);
        let mut mins = [(start, (0, Vec::new()))]
            .into_iter()
            .collect::<HashMap<_, _>>();
        let mut bh = [(Reverse(0), start)].into_iter().collect::<BinaryHeap<_>>();
        while let Some((Reverse(score), current)) = bh.pop() {
            if score != mins[&current].0 {
                continue;
            }
            for (next, next_score) in Self::neighbors(input, current, score) {
                match mins.entry(next) {
                    Entry::Occupied(mut e) => match next_score.cmp(&e.get().0) {
                        Ordering::Less => {
                            *e.get_mut() = (next_score, vec![current]);
                            bh.push((Reverse(next_score), next));
                        }
                        Ordering::Equal => {
                            e.get_mut().1.push(current);
                        }
                        Ordering::Greater => {}
                    },
                    Entry::Vacant(e) => {
                        e.insert((next_score, vec![current]));
                        bh.push((Reverse(next_score), next));
                    }
                }
            }
        }
        mins
    }
    fn neighbors(
        input: &Input,
        ((i, j), dir): PosWithDir,
        score: u32,
    ) -> impl Iterator<Item = (PosWithDir, u32)> {
        (match dir {
            Direction::North => [
                (((i - 1, j), Direction::North), score + STEP),
                (((i, j - 1), Direction::West), score + TURN + STEP),
                (((i, j + 1), Direction::East), score + TURN + STEP),
            ],
            Direction::South => [
                (((i + 1, j), Direction::South), score + STEP),
                (((i, j - 1), Direction::West), score + TURN + STEP),
                (((i, j + 1), Direction::East), score + TURN + STEP),
            ],
            Direction::West => [
                (((i, j - 1), Direction::West), score + STEP),
                (((i - 1, j), Direction::North), score + TURN + STEP),
                (((i + 1, j), Direction::South), score + TURN + STEP),
            ],
            Direction::East => [
                (((i, j + 1), Direction::East), score + 1),
                (((i + 1, j), Direction::South), score + 1001),
                (((i - 1, j), Direction::North), score + 1001),
            ],
        })
        .into_iter()
        .filter(|(((i, j), _), _)| input.maze[*i][*j])
    }
    fn find_min_score(
        mins: &HashMap<PosWithDir, (u32, Vec<PosWithDir>)>,
        end: (usize, usize),
    ) -> Option<u32> {
        mins.iter()
            .filter_map(|(&(p, _), &(score, _))| if p == end { Some(score) } else { None })
            .min()
    }
}

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = u32;
    type Answer2 = usize;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        Self::find_min_score(&Self::dijkstra_with_paths(input), input.end).unwrap()
    }
    fn part2(input: &Self::Input) -> Self::Answer2 {
        let mins = Self::dijkstra_with_paths(input);
        let min = Self::find_min_score(&mins, input.end).unwrap();
        let mut visited = HashSet::new();
        let mut vd = mins
            .keys()
            .filter(|p| p.0 == input.end && mins[p].0 == min)
            .collect::<VecDeque<_>>();
        while let Some(p) = vd.pop_front() {
            visited.insert(p.0);
            if let Some((_, v)) = mins.get(p) {
                for prev in v {
                    vd.push_back(prev);
                }
            }
        }
        visited.len()
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
        .trim_start()
        .parse()
    }

    fn example_input_second() -> Result<Input, Error> {
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
"
        .trim_start()
        .parse()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::part1(&example_input()?), 7036);
        assert_eq!(Solution::part1(&example_input_second()?), 11048);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::part2(&example_input()?), 45);
        assert_eq!(Solution::part2(&example_input_second()?), 64);
        Ok(())
    }
}
