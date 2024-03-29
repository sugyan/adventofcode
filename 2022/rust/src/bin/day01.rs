use aoc2022::Solve;
use itertools::Itertools;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    sorted_calories: Vec<u32>,
}

impl Solution {
    fn sum_top_n(&self, n: usize) -> u32 {
        self.sorted_calories.iter().rev().take(n).sum()
    }
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        Self {
            sorted_calories: BufReader::new(r)
                .lines()
                .filter_map(Result::ok)
                .collect_vec()
                .split(String::is_empty)
                .map(|lines| {
                    lines
                        .iter()
                        .filter_map(|line| line.parse::<u32>().ok())
                        .sum()
                })
                .sorted()
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.sum_top_n(1)
    }
    fn part2(&self) -> Self::Answer2 {
        self.sum_top_n(3)
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
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(24000, Solution::new(example_input()).part1());
    }

    #[test]
    fn part2() {
        assert_eq!(45000, Solution::new(example_input()).part2());
    }
}
