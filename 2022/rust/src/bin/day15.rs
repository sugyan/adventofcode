use aoc2022::Solve;
use itertools::Itertools;
use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

struct Coordinate {
    x: i64,
    y: i64,
}

impl Coordinate {
    fn distance(&self, rhs: &Self) -> i64 {
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

struct Solution {
    reports: Vec<(Coordinate, Coordinate)>,
}

impl Solution {
    fn ranges(&self, y: i64) -> Vec<(i64, i64)> {
        let mut ret: Vec<(i64, i64)> = Vec::new();
        for (min, max) in self
            .reports
            .iter()
            .filter_map(|(s, b)| {
                Some(s.distance(b) - (y - s.y).abs()).and_then(|r| {
                    if r >= 0 {
                        Some((s.x - r, s.x + r))
                    } else {
                        None
                    }
                })
            })
            .sorted_unstable()
        {
            if let Some(last) = ret.last_mut() {
                if last.1 >= min - 1 {
                    last.1 = max.max(last.1);
                    continue;
                }
            }
            ret.push((min, max));
        }
        ret
    }
}

impl Solve for Solution {
    type Answer1 = usize;
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
        self.ranges(target_row)
            .iter()
            .map(|r| {
                (r.1 - r.0) as usize + 1 - xs.iter().filter(|x| (r.0..=r.1).contains(x)).count()
            })
            .sum()
    }
    fn part2(&self) -> Self::Answer2 {
        let ymax = if cfg!(test) { 20 } else { 4_000_000 };
        let mut ps = Vec::with_capacity(self.reports.len() * 2);
        let mut ns = Vec::with_capacity(self.reports.len() * 2);
        for (s, b) in &self.reports {
            let d = s.distance(b);
            ps.extend([s.y - s.x + (d + 1), s.y - s.x - (d + 1)]);
            ns.extend([s.y + s.x + (d + 1), s.y + s.x - (d + 1)]);
        }
        let p = ps
            .iter()
            .sorted()
            .dedup_with_count()
            .filter_map(|(count, &b)| if count > 1 { Some(b) } else { None });
        let n = ns
            .iter()
            .sorted()
            .dedup_with_count()
            .filter_map(|(count, &b)| if count > 1 { Some(b) } else { None });
        p.cartesian_product(n)
            .filter_map(|b| Some((b.0 + b.1) / 2).filter(|y| (0..=ymax).contains(y)))
            .find_map(|y| {
                let v = self.ranges(y);
                if v.len() == 2 {
                    Some(y + (v[0].1 + 1) * 4_000_000)
                } else {
                    None
                }
            })
            .unwrap()
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
