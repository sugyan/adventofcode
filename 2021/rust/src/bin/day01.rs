use aoc2022::Solve;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    reports: Vec<u32>,
}

impl Solution {
    fn count_increases(&self, size: usize) -> usize {
        (size..self.reports.len())
            .filter(|&i| self.reports[i] > self.reports[i - size])
            .count()
    }
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;

    fn new(r: impl Read) -> Self {
        Self {
            reports: BufReader::new(r)
                .lines()
                .filter_map(Result::ok)
                .filter_map(|s| s.parse().ok())
                .collect::<Vec<_>>(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.count_increases(1)
    }
    fn part2(&self) -> Self::Answer2 {
        self.count_increases(3)
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
199
200
208
210
200
207
240
269
260
263"[1..]
            .as_bytes()
    }

    #[test]
    fn example_1() {
        assert_eq!(7, Solution::new(example_input()).part1());
    }

    #[test]
    fn example_2() {
        assert_eq!(5, Solution::new(example_input()).part2());
    }
}
