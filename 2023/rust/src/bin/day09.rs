use aoc2023::Solve;
use itertools::Itertools;
use std::io::{BufRead, BufReader, Read};

fn next_value(history: impl Iterator<Item = i32>) -> i32 {
    let mut stack = vec![Vec::from_iter(history)];
    while let Some(last) = stack.last() {
        if last.iter().all(|h| *h == 0) {
            break;
        }
        stack.push(
            last.iter()
                .collect_vec()
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect(),
        );
    }
    let mut value = 0;
    while stack.pop().is_some() {
        if let Some(last) = stack.last_mut() {
            value += last.last().expect("should have last element");
            last.push(value);
        }
    }
    value
}

struct Solution {
    histories: Vec<Vec<i32>>,
}

impl Solve for Solution {
    type Answer1 = i32;
    type Answer2 = i32;

    fn new(r: impl Read) -> Self {
        Self {
            histories: BufReader::new(r)
                .lines()
                .map_while(Result::ok)
                .map(|line| {
                    line.split_ascii_whitespace()
                        .map(|s| s.parse().expect("should be valid integer"))
                        .collect()
                })
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.histories
            .iter()
            .map(|history| next_value(history.iter().copied()))
            .sum()
    }
    fn part2(&self) -> Self::Answer2 {
        self.histories
            .iter()
            .map(|history| next_value(history.iter().copied().rev()))
            .sum()
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
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(Solution::new(example_input()).part1(), 114);
    }

    #[test]
    fn part2() {
        assert_eq!(Solution::new(example_input()).part2(), 2);
    }
}
