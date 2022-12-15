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
    type Answer2 = i32;

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
        let beacon_xs = self
            .reports
            .iter()
            .filter_map(|(_, b)| if b.y == target_row { Some(b.x) } else { None })
            .collect::<HashSet<_>>();
        let mut v = Vec::new();
        for &(s, b) in &self.reports {
            let d = s - b;
            let r = d.x.abs() + d.y.abs() - (target_row - s.y).abs();
            if r > 0 {
                v.push((s.x - r, s.x + r));
            }
        }
        v.sort_by(|a, b| match a.0.cmp(&b.0) {
            Ordering::Equal => b.1.cmp(&a.1),
            o => o,
        });
        let mut ret = 0;
        let (mut min, mut max) = v[0];
        for &range in v.iter().skip(1) {
            if range.0 > max {
                ret += max - min + 1;
                ret -= beacon_xs.iter().filter(|x| (min..=max).contains(x)).count() as i32;
                (min, max) = range;
            } else {
                max = max.max(range.1);
            }
        }
        ret += max - min + 1;
        ret -= beacon_xs.iter().filter(|x| (min..=max).contains(x)).count() as i32;
        ret
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
}
