use aoc2021::Solve;
use itertools::Itertools;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    lines: Vec<((i32, i32), (i32, i32))>,
}

impl Solution {
    fn count_overlapping(&self, diagonal: bool) -> usize {
        let mut points = HashMap::new();
        for &((x1, y1), (x2, y2)) in &self.lines {
            if !diagonal && x1 != x2 && y1 != y2 {
                continue;
            }
            let mut xy = (x1, y1);
            let d = ((x2 - x1).signum(), (y2 - y1).signum());
            while xy != (x2 + d.0, y2 + d.1) {
                *points.entry(xy).or_insert(0) += 1;
                xy.0 += d.0;
                xy.1 += d.1;
            }
        }
        points.values().filter(|&&v| v > 1).count()
    }
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;

    fn new(r: impl Read) -> Self {
        Self {
            lines: BufReader::new(r)
                .lines()
                .filter_map(Result::ok)
                .map(|line| {
                    line.split(" -> ")
                        .map(|s| {
                            s.split(',')
                                .map(|s| s.parse().unwrap())
                                .collect_tuple()
                                .unwrap()
                        })
                        .collect_tuple()
                        .unwrap()
                })
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.count_overlapping(false)
    }
    fn part2(&self) -> Self::Answer2 {
        self.count_overlapping(true)
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
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"[1..]
            .as_bytes()
    }

    #[test]
    fn example1() {
        assert_eq!(5, Solution::new(example_input()).part1());
    }

    #[test]
    fn example2() {
        assert_eq!(12, Solution::new(example_input()).part2());
    }
}
