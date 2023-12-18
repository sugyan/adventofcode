use aoc2023::Solve;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Dig {
    direction: Direction,
    meters: i64,
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
            color: c.trim_matches(|c| c == '(' || c == ')')[1..].to_string(),
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
    fn bfs(trenches: &HashSet<(i32, i32)>, (i, j): (i32, i32)) -> HashSet<(i32, i32)> {
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
        visited
    }
    fn cubic_meters(digs: impl Iterator<Item = (Direction, i64)>) -> i64 {
        // collect positions
        let (mut i, mut j) = (0, 0);
        let mut v = vec![(0, 0)];
        for (direction, meters) in digs {
            match direction {
                Direction::Up => i -= meters,
                Direction::Down => i += meters,
                Direction::Left => j -= meters,
                Direction::Right => j += meters,
            }
            v.push((i, j));
        }
        // map positions to ordered even integers
        let imap = v
            .iter()
            .map(|(i, _)| i)
            .unique()
            .sorted()
            .zip((0..).step_by(2))
            .collect::<HashMap<_, _>>();
        let jmap = v
            .iter()
            .map(|(_, j)| j)
            .unique()
            .sorted()
            .zip((0..).step_by(2))
            .collect::<HashMap<_, _>>();
        // create mapped trenches
        let mut mapped_trenches = HashSet::new();
        for w in v.windows(2) {
            let (i0, j0) = (imap[&w[0].0], jmap[&w[0].1]);
            let (i1, j1) = (imap[&w[1].0], jmap[&w[1].1]);
            for (i, j) in (i0.min(i1)..=i0.max(i1)).cartesian_product(j0.min(j1)..=j0.max(j1)) {
                mapped_trenches.insert((i, j));
            }
        }
        // find interior point around origin
        let (i0, j0) = (imap[&0], jmap[&0]);
        let p = [
            (i0 + 1, j0 + 1),
            (i0 - 1, j0 + 1),
            (i0 - 1, j0 - 1),
            (i0 + 1, j0 - 1),
        ]
        .into_iter()
        .find(|p| Self::is_interior(&mapped_trenches, p))
        .expect("should have at least one interior point");
        // expand mapped areas and calculate cubic meters
        let irevmap = imap.iter().map(|(k, v)| (v, *k)).collect::<HashMap<_, _>>();
        let jrevmap = jmap.iter().map(|(k, v)| (v, *k)).collect::<HashMap<_, _>>();
        mapped_trenches
            .union(&Self::bfs(&mapped_trenches, p))
            .map(|(i, j)| {
                (if i % 2 == 0 {
                    1
                } else {
                    irevmap[&(i + 1)] - irevmap[&(i - 1)] - 1
                }) * (if j % 2 == 0 {
                    1
                } else {
                    jrevmap[&(j + 1)] - jrevmap[&(j - 1)] - 1
                })
            })
            .sum()
    }
}

impl Solve for Solution {
    type Answer1 = i64;
    type Answer2 = i64;

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
        Self::cubic_meters(self.plan.iter().map(|d| (d.direction, d.meters)))
    }
    fn part2(&self) -> Self::Answer2 {
        Self::cubic_meters(self.plan.iter().map(|d| {
            (
                match &d.color[5..] {
                    "0" => Direction::Right,
                    "1" => Direction::Down,
                    "2" => Direction::Left,
                    "3" => Direction::Up,
                    _ => unreachable!(),
                },
                i64::from_str_radix(&d.color[..5], 16).expect("should be valid hexadecimal digits"),
            )
        }))
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

    #[test]
    fn test_part2() {
        assert_eq!(Solution::new(example_input()).part2(), 952_408_144_115);
    }
}
