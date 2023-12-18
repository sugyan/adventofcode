use std::collections::{HashSet, VecDeque};
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

use aoc2023::Solve;
use itertools::Itertools;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Dig {
    direction: Direction,
    meters: u32,
    #[allow(dead_code)]
    color: String,
}

impl FromStr for Dig {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (d, m, c) = s.split_ascii_whitespace().collect_tuple().ok_or(())?;
        Ok(Self {
            direction: match d {
                "R" => Direction::Right,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "U" => Direction::Up,
                _ => return Err(()),
            },
            meters: m.parse().map_err(|_| ())?,
            color: c.trim_matches(|c| c == '(' || c == ')').to_string(),
        })
    }
}

struct Solution {
    plan: Vec<Dig>,
}

impl Solution {
    fn is_interior(trenches: &HashSet<(i32, i32)>, (i, j): &(i32, i32)) -> bool {
        if let Some(max) = trenches.iter().map(|(i, _)| i).max() {
            return (*i..=*max).filter(|i| trenches.contains(&(*i, *j))).count() % 2 == 1;
        }
        false
    }
    fn bfs(trenches: &HashSet<(i32, i32)>, (i, j): (i32, i32)) -> usize {
        let mut visited = HashSet::new();
        let mut vd = VecDeque::from([(i, j)]);
        while let Some((i, j)) = vd.pop_front() {
            if trenches.contains(&(i, j)) {
                continue;
            }
            for p in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
                if !trenches.contains(&p) && visited.insert(p) {
                    vd.push_back(p);
                }
            }
        }
        visited.len()
    }
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        Self {
            plan: BufReader::new(r)
                .lines()
                .map_while(Result::ok)
                .map(|s| s.parse().expect("should be valid line"))
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let mut hs = HashSet::new();
        let (mut i, mut j) = (0, 0);
        for dig in &self.plan {
            for _ in 0..dig.meters {
                match dig.direction {
                    Direction::Up => i -= 1,
                    Direction::Down => i += 1,
                    Direction::Left => j -= 1,
                    Direction::Right => j += 1,
                }
                hs.insert((i, j));
            }
        }
        [(1, 1), (-1, 1), (-1, -1), (1, -1)]
            .iter()
            .find_map(|p| {
                if Self::is_interior(&hs, p) {
                    Some(hs.len() + Self::bfs(&hs, *p))
                } else {
                    None
                }
            })
            .expect("should have interior position")
    }
    fn part2(&self) -> Self::Answer2 {
        todo!()
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
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
"[1..]
            .as_bytes()
    }

    #[test]
    fn test_part1() {
        assert_eq!(Solution::new(example_input()).part1(), 62);
    }
}
