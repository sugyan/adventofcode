use aoc2022::Solve;
use itertools::Itertools;
use std::io::{BufRead, BufReader};

struct Solution {
    pairs: Vec<((u32, u32), (u32, u32))>,
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;

    fn new(r: impl std::io::Read) -> Self {
        Self {
            pairs: BufReader::new(r)
                .lines()
                .filter_map(Result::ok)
                .filter_map(|s| {
                    s.split(',')
                        .filter_map(|s| s.split('-').filter_map(|s| s.parse().ok()).collect_tuple())
                        .collect_tuple()
                })
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.pairs
            .iter()
            .filter(|(a0, a1)| (a0.0 <= a1.0 && a1.1 <= a0.1) || (a1.0 <= a0.0 && a0.1 <= a1.1))
            .count()
    }
    fn part2(&self) -> Self::Answer2 {
        self.pairs
            .iter()
            .filter(|(a0, a1)| a0.0.max(a1.0) <= a0.1.min(a1.1))
            .count()
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

    #[test]
    fn example2() {
        assert_eq!(4, Solution::new(example_input()).part2());
    }
}
