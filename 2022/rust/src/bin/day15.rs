use aoc2022::Solve;
use itertools::Itertools;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

struct Coordinate {
    x: i64,
    y: i64,
}

struct Report {
    sensor: Coordinate,
    beacon: Coordinate,
    distance: i64,
}

impl FromStr for Report {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(": ")
            .and_then(|(s, b)| {
                [&s[10..], &b[21..]]
                    .iter()
                    .filter_map(|s| {
                        s.split_once(", ").and_then(|(x, y)| {
                            Some(Coordinate {
                                x: x[2..].parse().ok()?,
                                y: y[2..].parse().ok()?,
                            })
                        })
                    })
                    .collect_tuple()
                    .map(|(sensor, beacon)| {
                        let distance = (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs();
                        Report {
                            sensor,
                            beacon,
                            distance,
                        }
                    })
            })
            .ok_or(())
    }
}

struct Solution {
    reports: Vec<Report>,
}

impl Solution {
    fn ranges(&self, y: i64) -> Vec<(i64, i64)> {
        let mut ret: Vec<(i64, i64)> = Vec::new();
        for (min, max) in self
            .reports
            .iter()
            .filter_map(|report| {
                Some(report.distance - (y - report.sensor.y).abs()).and_then(|r| {
                    if r >= 0 {
                        Some((report.sensor.x - r, report.sensor.x + r))
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
    type Answer1 = i64;
    type Answer2 = i64;

    fn new(r: impl Read) -> Self {
        Self {
            reports: BufReader::new(r)
                .lines()
                .filter_map(Result::ok)
                .filter_map(|line| line.parse().ok())
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let target_row = if cfg!(test) { 10 } else { 2_000_000 };
        let count = self
            .reports
            .iter()
            .filter_map(|report| {
                if report.beacon.y == target_row {
                    Some(report.beacon.x)
                } else {
                    None
                }
            })
            .unique()
            .count() as i64;
        self.ranges(target_row)
            .iter()
            .map(|r| r.1 - r.0 + 1)
            .sum::<i64>()
            - count
    }
    fn part2(&self) -> Self::Answer2 {
        let max = if cfg!(test) { 20 } else { 4_000_000 };
        let mut ps = vec![0];
        let mut ns = vec![max];
        for report in &self.reports {
            ps.push(report.sensor.y - report.sensor.x + (report.distance + 1));
            ps.push(report.sensor.y - report.sensor.x - (report.distance + 1));
            ns.push(report.sensor.y + report.sensor.x + (report.distance + 1));
            ns.push(report.sensor.y + report.sensor.x - (report.distance + 1));
        }
        ps.iter()
            .cartesian_product(&ns)
            .filter_map(|b| Some((b.0 + b.1) / 2).filter(|y| (0..=max).contains(y)))
            .find_map(|y| {
                let v = self.ranges(y);
                let x = v[0].1 + 1;
                if v.len() == 2 && (0..=max).contains(&x) {
                    Some(x * 4_000_000 + y)
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
