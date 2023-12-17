use aoc2023::Solve;
use itertools::Itertools;
use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read};

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl Direction {
    fn next_position(&self, (i, j): (usize, usize)) -> (usize, usize) {
        match self {
            Self::Right => (i, j.wrapping_add(1)),
            Self::Up => (i.wrapping_sub(1), j),
            Self::Left => (i, j.wrapping_sub(1)),
            Self::Down => (i.wrapping_add(1), j),
        }
    }
    fn next_directions(&self, tile: char) -> Vec<Self> {
        match (self, tile) {
            (Self::Right | Self::Left, '|') => vec![Self::Up, Self::Down],
            (Self::Up | Self::Down, '-') => vec![Self::Right, Self::Left],
            (d, '/') => vec![match d {
                Self::Right => Self::Up,
                Self::Up => Self::Right,
                Self::Left => Self::Down,
                Self::Down => Self::Left,
            }],
            (d, '\\') => vec![match d {
                Self::Right => Self::Down,
                Self::Up => Self::Left,
                Self::Left => Self::Up,
                Self::Down => Self::Right,
            }],
            _ => vec![*self],
        }
    }
}

struct Solution {
    contraption: Vec<Vec<char>>,
}

impl Solution {
    fn energized_count(&self, start: (usize, usize), direction: Direction) -> usize {
        let (rows, cols) = (self.contraption.len(), self.contraption[0].len());
        let mut seen = HashSet::new();
        let mut beams = vec![(start, direction)];
        while let Some((position, direction)) = beams.pop() {
            let (i, j) = direction.next_position(position);
            if !(0..rows).contains(&i) || !(0..cols).contains(&j) {
                continue;
            }
            if !seen.insert(((i, j), direction)) {
                continue;
            }
            for d in direction.next_directions(self.contraption[i][j]) {
                beams.push(((i, j), d));
            }
        }
        seen.iter().map(|((i, j), _)| (i, j)).unique().count()
    }
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;

    fn new(r: impl Read) -> Self {
        Self {
            contraption: BufReader::new(r)
                .lines()
                .map_while(Result::ok)
                .map(|s| s.chars().collect())
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.energized_count((0, !0), Direction::Right)
    }
    fn part2(&self) -> Self::Answer2 {
        let (rows, cols) = (self.contraption.len(), self.contraption[0].len());
        [
            (0..cols).map(|j| ((!0, j), Direction::Down)).collect_vec(),
            (0..cols).map(|j| ((rows, j), Direction::Up)).collect(),
            (0..rows).map(|i| ((i, !0), Direction::Right)).collect(),
            (0..rows).map(|i| ((i, cols), Direction::Left)).collect(),
        ]
        .concat()
        .iter()
        .map(|&(start, direction)| self.energized_count(start, direction))
        .max()
        .expect("should have max value")
    }
}

fn main() {
    let solution = Solution::new(std::io::stdin().lock());
    println!("Part 1: {}", solution.part1());
    println!("Part 2: {}", solution.part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static [u8] {
        r"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....        
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(Solution::new(example_input()).part1(), 46);
    }

    #[test]
    fn part2() {
        assert_eq!(Solution::new(example_input()).part2(), 51);
    }
}
