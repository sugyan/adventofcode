use aoc2022::Solve;
use itertools::Itertools;
use std::collections::BTreeSet;
use std::io::{BufRead, BufReader};

struct Solution {
    paths: Vec<Vec<(i32, i32)>>,
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl std::io::Read) -> Self {
        Self {
            paths: BufReader::new(r)
                .lines()
                .filter_map(Result::ok)
                .map(|line| {
                    line.split(" -> ")
                        .filter_map(|xy| {
                            xy.split(',').filter_map(|s| s.parse().ok()).collect_tuple()
                        })
                        .collect()
                })
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let mut bts = BTreeSet::new();
        for path in &self.paths {
            for w in path.windows(2) {
                if w[0].0 == w[1].0 {
                    for y in w[0].1.min(w[1].1)..=w[0].1.max(w[1].1) {
                        bts.insert((y, w[0].0));
                    }
                }
                if w[0].1 == w[1].1 {
                    for x in w[0].0.min(w[1].0)..=w[0].0.max(w[1].0) {
                        bts.insert((w[0].1, x));
                    }
                }
            }
        }
        let ymax = bts.iter().last().unwrap().0;
        for i in 0.. {
            let mut sand = (500, 0);
            loop {
                if !bts.contains(&(sand.1 + 1, sand.0)) {
                    sand.1 += 1;
                } else if !bts.contains(&(sand.1 + 1, sand.0 - 1)) {
                    sand.1 += 1;
                    sand.0 -= 1;
                } else if !bts.contains(&(sand.1 + 1, sand.0 + 1)) {
                    sand.1 += 1;
                    sand.0 += 1;
                } else {
                    break;
                }
                if sand.1 > ymax {
                    return i;
                }
            }
            bts.insert((sand.1, sand.0));
        }
        unreachable!();
    }
    fn part2(&self) -> Self::Answer2 {
        let mut bts = BTreeSet::new();
        for path in &self.paths {
            for w in path.windows(2) {
                if w[0].0 == w[1].0 {
                    for y in w[0].1.min(w[1].1)..=w[0].1.max(w[1].1) {
                        bts.insert((y, w[0].0));
                    }
                }
                if w[0].1 == w[1].1 {
                    for x in w[0].0.min(w[1].0)..=w[0].0.max(w[1].0) {
                        bts.insert((w[0].1, x));
                    }
                }
            }
        }
        let ymax = bts.iter().last().unwrap().0;
        for i in 0.. {
            let mut sand = (500, 0);
            if bts.contains(&(sand.1, sand.0)) {
                return i;
            }
            while sand.1 < ymax + 1 {
                if !bts.contains(&(sand.1 + 1, sand.0)) {
                    sand.1 += 1;
                } else if !bts.contains(&(sand.1 + 1, sand.0 - 1)) {
                    sand.1 += 1;
                    sand.0 -= 1;
                } else if !bts.contains(&(sand.1 + 1, sand.0 + 1)) {
                    sand.1 += 1;
                    sand.0 += 1;
                } else {
                    break;
                }
            }
            bts.insert((sand.1, sand.0));
        }
        unreachable!();
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
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(24, Solution::new(example_input()).part1());
    }

    #[test]
    fn part2() {
        assert_eq!(93, Solution::new(example_input()).part2());
    }
}
