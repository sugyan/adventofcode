use aoc2023::Solve;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    input: Vec<String>,
}

impl Solution {
    fn calibration_value(line: &str, include_spelled: bool) -> u32 {
        let mut digits = (1..=9).map(|i| (i.to_string(), i)).collect::<Vec<_>>();
        if include_spelled {
            digits.extend([
                ("one".into(), 1),
                ("two".into(), 2),
                ("three".into(), 3),
                ("four".into(), 4),
                ("five".into(), 5),
                ("six".into(), 6),
                ("seven".into(), 7),
                ("eight".into(), 8),
                ("nine".into(), 9),
            ]);
        }
        let mut v = Vec::new();
        for i in 0..line.len() {
            for (s, u) in &digits {
                if line[i..].starts_with(s) {
                    v.push(*u);
                }
            }
        }
        v[0] * 10 + v[v.len() - 1]
    }
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        Self {
            input: BufReader::new(r).lines().map_while(Result::ok).collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.input
            .iter()
            .map(|s| Self::calibration_value(s, false))
            .sum()
    }
    fn part2(&self) -> Self::Answer2 {
        self.input
            .iter()
            .map(|s| Self::calibration_value(s, true))
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

    #[test]
    fn part1() {
        assert_eq!(
            142,
            Solution::new(
                r"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"[1..]
                    .as_bytes()
            )
            .part1()
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            281,
            Solution::new(
                r"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"[1..]
                    .as_bytes()
            )
            .part2()
        );
    }
}
