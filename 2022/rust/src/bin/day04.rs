use std::io::{BufRead, BufReader};

use aoc2022::Solve;
use itertools::Itertools;

struct Solution {
    assignments: Vec<Vec<(u32, u32)>>,
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;

    fn new(r: impl std::io::Read) -> Self {
        Self {
            assignments: BufReader::new(r)
                .lines()
                .filter_map(Result::ok)
                .map(|s| {
                    s.split(',')
                        .filter_map(|s| s.split('-').filter_map(|s| s.parse().ok()).collect_tuple())
                        .collect()
                })
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.assignments
            .iter()
            .filter(|a| a.contains(&(a[0].0.max(a[1].0), a[0].1.min(a[1].1))))
            .count()
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
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"[1..]
            .as_bytes()
    }

    #[test]
    fn example1() {
        assert_eq!(2, Solution::new(example_input()).part1());
    }
}
