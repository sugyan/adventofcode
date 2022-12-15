use aoc2022::Solve;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read};
use std::ops::Sub;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn distance(self, rhs: Self) -> i32 {
        (self.x - rhs.x).abs() + (self.y - rhs.y).abs()
    }
}

impl FromStr for Coordinate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(", ")
            .and_then(|(x, y)| {
                Some(Self {
                    x: x[2..].parse().ok()?,
                    y: y[2..].parse().ok()?,
                })
            })
            .ok_or(())
    }
}

impl Sub for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

struct Solution {
    reports: Vec<(Coordinate, Coordinate)>,
}

impl Solve for Solution {
    type Answer1 = i32;
    type Answer2 = i64;

    fn new(r: impl Read) -> Self {
        Self {
            reports: BufReader::new(r)
                .lines()
                .filter_map(Result::ok)
                .filter_map(|line| {
                    line.split_once(": ").and_then(|(s, b)| {
                        [&s[10..], &b[21..]]
                            .iter()
                            .filter_map(|s| s.parse().ok())
                            .collect_tuple()
                    })
                })
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let target_row = if cfg!(test) { 10 } else { 2_000_000 };
        let xs = self
            .reports
            .iter()
            .filter_map(|(_, b)| if b.y == target_row { Some(b.x) } else { None })
            .collect::<HashSet<_>>();
        let mut v = self
            .reports
            .iter()
            .filter_map(|&(s, b)| match s.distance(b) - (target_row - s.y).abs() {
                i32::MIN..=0 => None,
                r => Some((s.x - r, s.x + r)),
            })
            .collect::<Vec<_>>();
        v.sort_by(|a, b| match b.0.cmp(&a.0) {
            Ordering::Equal => a.1.cmp(&b.1),
            o => o,
        });
        let mut ret = 0;
        while let Some((min, max)) = v.pop() {
            if let Some(last) = v.last_mut() {
                if last.0 <= max {
                    *last = (min.min(last.0), max.max(last.1));
                    continue;
                }
            }
            ret += max - min + 1 - xs.iter().filter(|x| (min..=max).contains(x)).count() as i32;
        }
        ret
    }
    fn part2(&self) -> Self::Answer2 {
        let mut v = Vec::new();
        for c in self.reports.iter().combinations(2) {
            let c0 = c[0];
            let c1 = c[1];
            if c0.0.distance(c1.0) == c0.0.distance(c0.1) + c1.0.distance(c1.1) + 2 {
                let edges = (
                    (
                        c0.0.x + (c0.0.distance(c0.1) + 1) * (c1.0.x - c0.0.x).signum(),
                        c0.0.y,
                    ),
                    (
                        c0.0.x,
                        c0.0.y + (c0.0.distance(c0.1) + 1) * (c1.0.y - c0.0.y).signum(),
                    ),
                    (
                        c1.0.x + (c1.0.distance(c1.1) + 1) * (c0.0.x - c1.0.x).signum(),
                        c1.0.y,
                    ),
                    (
                        c1.0.x,
                        c1.0.y + (c1.0.distance(c1.1) + 1) * (c0.0.y - c1.0.y).signum(),
                    ),
                );
                v.push(if c0.0.x.cmp(&c1.0.x) == c0.0.y.cmp(&c1.0.y) {
                    (
                        (
                            edges.0 .0.min(edges.1 .0).max(edges.2 .0.min(edges.3 .0)),
                            edges.0 .1.max(edges.1 .1).min(edges.2 .1.max(edges.3 .1)),
                        ),
                        (
                            edges.0 .0.max(edges.1 .0).min(edges.2 .0.max(edges.3 .0)),
                            edges.0 .1.min(edges.1 .1).max(edges.2 .1.min(edges.3 .1)),
                        ),
                    )
                } else {
                    (
                        (
                            edges.0 .0.min(edges.1 .0).max(edges.2 .0.min(edges.3 .0)),
                            edges.0 .1.min(edges.1 .1).max(edges.2 .1.min(edges.3 .1)),
                        ),
                        (
                            edges.0 .0.max(edges.1 .0).min(edges.2 .0.max(edges.3 .0)),
                            edges.0 .1.max(edges.1 .1).min(edges.2 .1.max(edges.3 .1)),
                        ),
                    )
                });
            }
        }
        let position = v
            .iter()
            .combinations(2)
            .filter_map(|c| {
                if c[0].0 .1.cmp(&c[0].1 .1) == c[1].0 .1.cmp(&c[1].1 .1) {
                    return None;
                }
                let b1 = if c[0].0 .1 < c[0].1 .1 {
                    c[0].0 .1 - c[0].0 .0
                } else {
                    c[1].0 .1 - c[1].0 .0
                };
                let b2 = if c[1].0 .1 > c[1].1 .1 {
                    c[1].0 .1 + c[1].0 .0
                } else {
                    c[0].0 .1 + c[0].0 .0
                };
                Some(Coordinate {
                    x: (b2 - b1) / 2,
                    y: (b2 + b1) / 2,
                })
            })
            .find(|&p| {
                self.reports
                    .iter()
                    .all(|&(s, b)| s.distance(p) > s.distance(b))
            })
            .unwrap();
        position.x as i64 * 4_000_000 + position.y as i64
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
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(26, Solution::new(example_input()).part1());
    }

    #[test]
    fn part2() {
        assert_eq!(56_000_011, Solution::new(example_input()).part2());
    }
}
