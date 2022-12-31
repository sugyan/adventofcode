use aoc2022::Solve;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read};
use std::ops::Not;

enum Turn {
    R,
    L,
}

enum Instruction {
    Number(usize),
    Letter(Turn),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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
    fn common_next(&self) -> HashMap<(usize, usize, Direction), (usize, usize, Direction)> {
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
                    }
                }
            }
        }
        hm
    }
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
        let mut hm = self.common_next();
        for (i, j) in (0..rows)
            .cartesian_product(0..cols)
            .filter(|&(i, j)| self.map[i][j] == Some(true))
        {
            for d in Direction::ALL {
                let (di, dj) = d.delta();
                let ii = i.wrapping_add(di);
                let jj = j.wrapping_add(dj);
                if (0..rows).contains(&ii) && (0..cols).contains(&jj) && self.map[ii][jj].is_some()
                {
                    continue;
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
        let (rows, cols) = (self.map.len(), self.map[0].len());
        //    4 --- 5
        //   /|    /|
        //  / |   / |
        // 0 --- 1  |
        // |  7 -|- 6
        // | /   | /
        // |/    |/
        // 3 --- 2
        let surfaces = [
            [0, 1, 2, 3],
            [1, 5, 6, 2],
            [5, 4, 7, 6],
            [4, 0, 3, 7],
            [0, 4, 5, 1],
            [3, 2, 6, 7],
        ];
        let s = self
            .map
            .iter()
            .map(|row| row.iter().filter(|&col| col.is_some()).count())
            .sum::<usize>();
        let a = ((s / 6) as f32).sqrt() as usize;
        let mut stack = Vec::new();
        let (mut visited, mut connected) = (HashSet::new(), HashSet::new());
        let mut verteces = HashMap::new();
        if let Some(j) = self.map[0].iter().position(|&t| t.is_some()) {
            stack.push((
                [(0, j), (0, j + a - 1), (a - 1, j + a - 1), (a - 1, j)],
                (0, 1),
                0,
            ));
            visited.insert((0, j));
        }
        while let Some((p, e, r)) = stack.pop() {
            if let Some(s) = surfaces
                .iter()
                .find(|s| (0..4).any(|i| s[i] == e.0 && s[(i + 1) % 4] == e.1))
            {
                if let Some(i) = (0..4).find(|&i| s[i] == e.0 && s[(i + 1) % 4] == e.1) {
                    for j in 0..4 {
                        verteces.insert(p[j], s[(i + j + r) % 4]);
                    }
                    for (d, r0, r1) in [
                        ((0, a), 1, 1),
                        ((a, 0), 2, 0),
                        ((0, (!0_usize).wrapping_mul(a)), 3, 3),
                    ] {
                        let (ii, jj) = (p[0].0.wrapping_add(d.0), p[0].1.wrapping_add(d.1));
                        if (0..rows).contains(&ii)
                            && (0..cols).contains(&jj)
                            && self.map[ii][jj].is_some()
                            && !visited.contains(&(ii, jj))
                        {
                            let e = (s[(i + r + r0 + 1) % 4], s[(i + r + r0) % 4]);
                            visited.insert((ii, jj));
                            stack.push((
                                [
                                    (p[0].0.wrapping_add(d.0), p[0].1.wrapping_add(d.1)),
                                    (p[1].0.wrapping_add(d.0), p[1].1.wrapping_add(d.1)),
                                    (p[2].0.wrapping_add(d.0), p[2].1.wrapping_add(d.1)),
                                    (p[3].0.wrapping_add(d.0), p[3].1.wrapping_add(d.1)),
                                ],
                                e,
                                r1,
                            ));
                            connected.insert((e.0.min(e.1), e.0.max(e.1)));
                        }
                    }
                }
            }
        }
        let mut edges = HashMap::new();
        for s in visited {
            for (i, d) in [(0, 0), (0, a - 1), (a - 1, a - 1), (a - 1, 0), (0, 0)]
                .windows(2)
                .enumerate()
            {
                let (e0, e1) = ((s.0 + d[0].0, s.1 + d[0].1), (s.0 + d[1].0, s.1 + d[1].1));
                let (v0, v1) = (verteces[&e0], verteces[&e1]);
                if connected.contains(&(v0.min(v1), v0.max(v1))) {
                    continue;
                }
                let (mut p, mut d) = (e0, Direction::ALL[i % 4]);
                if v0 > v1 {
                    (p, d) = (e1, !d);
                }
                let delta = d.delta();
                let mut v = Vec::new();
                for _ in 0..a {
                    v.push(p);
                    p = (p.0.wrapping_add(delta.0), p.1.wrapping_add(delta.1));
                }
                edges
                    .entry((v0.min(v1), v0.max(v1)))
                    .or_insert_with(Vec::new)
                    .push((v, Direction::ALL[(i + 3) % 4]));
            }
        }
        let mut hm = self.common_next();
        for v in edges.values() {
            for (p0, p1) in v[0].0.iter().zip(&v[1].0) {
                if self.map[p0.0][p0.1] == Some(true) && self.map[p1.0][p1.1] == Some(true) {
                    hm.insert((p0.0, p0.1, v[0].1), (p1.0, p1.1, !v[1].1));
                    hm.insert((p1.0, p1.1, v[1].1), (p0.0, p0.1, !v[0].1));
                }
            }
        }
        self.final_password(&hm)
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

    #[test]
    fn part2() {
        assert_eq!(5031, Solution::new(example_input()).part2());
    }
}
