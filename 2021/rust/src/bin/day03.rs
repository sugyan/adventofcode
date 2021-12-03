use std::io::{BufRead, BufReader};

struct Solution {
    bit_counts: Vec<(usize, usize)>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        let mut bit_counts = Vec::new();
        for report in inputs {
            for (i, c) in report.chars().rev().enumerate() {
                if bit_counts.len() <= i {
                    bit_counts.push((0, 0));
                }
                if c == '0' {
                    bit_counts[i].0 += 1;
                } else {
                    bit_counts[i].1 += 1;
                }
            }
        }
        Self { bit_counts }
    }
    fn part_1(&self) -> u32 {
        let gamma = self
            .bit_counts
            .iter()
            .rev()
            .fold(0, |acc, (c0, c1)| (acc << 1) + if c1 > c0 { 1 } else { 0 });
        gamma * (((1 << self.bit_counts.len()) - 1) ^ gamma)
    }
}

fn main() {
    let solution = Solution::new(
        &BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(Result::ok)
            .collect::<Vec<_>>(),
    );
    println!("{}", solution.part_1());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_inputs() -> Vec<String> {
        r"
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"[1..]
            .split('\n')
            .map(String::from)
            .collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(198, Solution::new(&example_inputs()).part_1());
    }
}
