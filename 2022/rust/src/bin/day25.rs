use std::io::{BufRead, BufReader};

use aoc2022::Solve;

struct Solution {
    numbers: Vec<String>,
}

impl Solve for Solution {
    type Answer1 = String;
    type Answer2 = String;

    fn new(r: impl std::io::Read) -> Self {
        Self {
            numbers: BufReader::new(r).lines().filter_map(Result::ok).collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let mut v = Vec::new();
        for number in &self.numbers {
            while number.len() > v.len() {
                v.push(0);
            }
            number.chars().rev().enumerate().for_each(|(i, c)| {
                v[i] += match c {
                    '0' => 0,
                    '1' => 1,
                    '2' => 2,
                    '-' => -1,
                    '=' => -2,
                    _ => unreachable!(),
                }
            });
        }
        let mut i = 0;
        while i < v.len() {
            let mut carry = 0;
            while v[i] > 2 {
                carry += 1;
                v[i] -= 5;
            }
            while v[i] < -2 {
                carry -= 1;
                v[i] += 5;
            }
            if carry != 0 {
                if i + 1 == v.len() {
                    v.push(0);
                }
                v[i + 1] += carry;
            }
            i += 1;
        }
        v.iter()
            .rev()
            .map(|n| match n {
                0 => '0',
                1 => '1',
                2 => '2',
                -1 => '-',
                -2 => '=',
                _ => unreachable!(),
            })
            .collect()
    }
    fn part2(&self) -> Self::Answer2 {
        unreachable!();
    }
}

fn main() {
    let solution = Solution::new(std::io::stdin().lock());
    println!("Part 1: {}", solution.part1());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static [u8] {
        r"
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!("2=-1=0", Solution::new(example_input()).part1());
    }
}
