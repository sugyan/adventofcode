use aoc2022::Solve;
use std::io::{BufRead, BufReader};

struct Solution {
    calories: Vec<Vec<u32>>,
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl std::io::Read) -> Self {
        Self {
            calories: BufReader::new(r)
                .lines()
                .filter_map(Result::ok)
                .collect::<Vec<_>>()
                .split(String::is_empty)
                .map(|lines| lines.iter().filter_map(|line| line.parse().ok()).collect())
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.calories.iter().map(|v| v.iter().sum()).max().unwrap()
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
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"[1..]
            .as_bytes()
    }

    #[test]
    fn example1() {
        assert_eq!(24000, Solution::new(example_input()).part1());
    }
}
