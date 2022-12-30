use aoc2022::Solve;
use itertools::Itertools;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::ops::Not;

#[derive(Debug)]
enum Turn {
    R,
    L,
}

#[derive(Debug)]
enum Instruction {
    Number(usize),
    Letter(Turn),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Direction {
    const ALL: [Direction; 4] = [
        Direction::Right,
        Direction::Down,
        Direction::Left,
        Direction::Up,
    ];
    fn delta(&self) -> (usize, usize) {
        match self {
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, !0),
            Direction::Up => (!0, 0),
        }
    }
    fn turn(&mut self, t: &Turn) {
        *self = match (*self, t) {
            (Direction::Up, Turn::R) | (Direction::Down, Turn::L) => Direction::Right,
            (Direction::Right, Turn::R) | (Direction::Left, Turn::L) => Direction::Down,
            (Direction::Down, Turn::R) | (Direction::Up, Turn::L) => Direction::Left,
            (Direction::Left, Turn::R) | (Direction::Right, Turn::L) => Direction::Up,
        }
    }
}

impl Not for Direction {
    type Output = Direction;

    fn not(self) -> Self::Output {
        match self {
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Up => Direction::Down,
        }
    }
}

struct Solution {
    map: Vec<Vec<Option<bool>>>,
    path: Vec<Instruction>,
}

impl Solution {
    fn final_password(
        &self,
        next: &HashMap<(usize, usize, Direction), (usize, usize, Direction)>,
    ) -> usize {
        let (mut i, mut j, mut d) = (
            0,
            self.map[0].iter().position(|&t| t == Some(true)).unwrap(),
            Direction::Right,
        );
        for inst in &self.path {
            match inst {
                Instruction::Number(n) => {
                    for _ in 0..*n {
                        if let Some(n) = next.get(&(i, j, d)) {
                            (i, j, d) = *n;
                        }
                    }
                }
                Instruction::Letter(t) => d.turn(t),
            }
        }
        (i + 1) * 1000 + (j + 1) * 4 + d as usize
    }
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;

    fn new(r: impl Read) -> Self {
        let lines = BufReader::new(r)
            .lines()
            .filter_map(Result::ok)
            .collect::<Vec<_>>();
        let parts = lines.split(String::is_empty).collect::<Vec<_>>();
        let w = parts[0].iter().map(|s| s.len()).max().unwrap();
        let map = parts[0]
            .iter()
            .map(|s| {
                (0..w)
                    .map(|i| match s.chars().nth(i) {
                        Some('.') => Some(true),
                        Some('#') => Some(false),
                        _ => None,
                    })
                    .collect()
            })
            .collect();
        let (mut path, mut s) = (Vec::new(), String::new());
        for c in parts[1][0].chars() {
            if c.is_numeric() {
                s.push(c);
            } else {
                if let Ok(n) = s.parse() {
                    path.push(Instruction::Number(n));
                }
                path.push(Instruction::Letter(match c {
                    'R' => Turn::R,
                    'L' => Turn::L,
                    _ => unreachable!(),
                }));
                s.clear();
            }
        }
        if let Ok(n) = s.parse() {
            path.push(Instruction::Number(n));
        }
        Self { map, path }
    }
    fn part1(&self) -> Self::Answer1 {
        let (rows, cols) = (self.map.len(), self.map[0].len());
        let mut hm = HashMap::new();
        for (i, j) in (0..rows)
            .cartesian_product(0..cols)
            .filter(|&(i, j)| self.map[i][j] == Some(true))
        {
            for d in Direction::ALL {
                let (di, dj) = d.delta();
                let ii = i.wrapping_add(di);
                let jj = j.wrapping_add(dj);
                if (0..rows).contains(&ii) && (0..cols).contains(&jj) {
                    if let Some(b) = self.map[ii][jj] {
                        if b {
                            hm.insert((i, j, d), (ii, jj, d));
                        }
                        continue;
                    }
                }
                let (mut pi, mut pj) = (i, j);
                let (di, dj) = (!d).delta();
                while {
                    let ii = pi.wrapping_add(di);
                    let jj = pj.wrapping_add(dj);
                    (0..rows).contains(&ii) && (0..cols).contains(&jj) && self.map[ii][jj].is_some()
                } {
                    pi = pi.wrapping_add(di);
                    pj = pj.wrapping_add(dj);
                }
                if let Some(b) = self.map[pi][pj] {
                    if b {
                        hm.insert((i, j, d), (pi, pj, d));
                    }
                }
            }
        }
        self.final_password(&hm)
    }
    fn part2(&self) -> Self::Answer2 {
        todo!()
    }
}

fn main() {
    let solution = Solution::new(std::io::stdin().lock());
    println!("Part 1: {}", solution.part1());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static [u8] {
        r"
        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(6032, Solution::new(example_input()).part1());
    }
}
