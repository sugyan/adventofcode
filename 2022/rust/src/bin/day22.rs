use aoc2022::Solve;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

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

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = s.parse() {
            Ok(Self::Number(n))
        } else {
            match s {
                "R" => Ok(Self::Letter(Turn::R)),
                "L" => Ok(Self::Letter(Turn::L)),
                _ => Err(()),
            }
        }
    }
}

struct Solution {
    map: Vec<Vec<Option<bool>>>,
    path: Vec<Instruction>,
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
                s.clear();
                if let Ok(inst) = String::from(c).parse() {
                    path.push(inst);
                }
            }
        }
        if let Ok(n) = s.parse() {
            path.push(Instruction::Number(n));
        }
        Self { map, path }
    }
    fn part1(&self) -> Self::Answer1 {
        let (rows, cols) = (self.map.len(), self.map[0].len());
        let directions = [(0, 1), (1, 0), (0, !0), (!0, 0)];
        let (mut i, mut j) = (
            0_usize,
            self.map[0].iter().position(|&t| t == Some(true)).unwrap(),
        );
        let mut d = 0;
        let next_pos = |(i, j): (usize, usize), d| {
            let (di, dj) = directions[d];
            let ii = i.wrapping_add(di);
            let jj = j.wrapping_add(dj);
            if (0..rows).contains(&ii) && (0..cols).contains(&jj) {
                match self.map[ii][jj] {
                    Some(true) => return (ii, jj),
                    Some(false) => return (i, j),
                    _ => {}
                }
            }
            let (mut pi, mut pj) = (i, j);
            let (di, dj) = directions[(d + 2) % 4];
            while {
                let ii = pi.wrapping_add(di);
                let jj = pj.wrapping_add(dj);
                (0..rows).contains(&ii) && (0..cols).contains(&jj) && self.map[ii][jj].is_some()
            } {
                pi = pi.wrapping_add(di);
                pj = pj.wrapping_add(dj);
            }
            if self.map[pi][pj].unwrap() {
                (pi, pj)
            } else {
                (i, j)
            }
        };
        for inst in &self.path {
            match inst {
                Instruction::Number(n) => {
                    for _ in 0..*n {
                        (i, j) = next_pos((i, j), d);
                    }
                }
                Instruction::Letter(Turn::R) => d = (d + 1) % 4,
                Instruction::Letter(Turn::L) => d = (d + 3) % 4,
            }
        }
        (i + 1) * 1000 + (j + 1) * 4 + d
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
