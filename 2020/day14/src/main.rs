use regex::Regex;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

struct Solution {
    inputs: Vec<String>,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        Self { inputs }
    }
    fn solve_1(&self) -> u64 {
        let re = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
        let mut masks: (u64, u64) = ((1 << 36) - 1, 0);
        let mut mem: HashMap<u64, u64> = HashMap::new();
        for input in self.inputs.iter() {
            if let Some(mask) = input.strip_prefix("mask = ") {
                masks = ((1 << 36) - 1, 0);
                for (i, c) in mask.chars().rev().enumerate() {
                    match c {
                        '0' => masks.0 &= !(1 << i),
                        '1' => masks.1 |= 1 << i,
                        _ => {}
                    }
                }
            } else if let Some(cap) = re.captures(input) {
                if let (Ok(address), Ok(value)) = (cap[1].parse::<u64>(), cap[2].parse::<u64>()) {
                    mem.insert(address, value & masks.0 | masks.1);
                }
            }
        }
        mem.values().sum()
    }
}

fn main() {
    let solution = Solution::new(
        BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(|line| line.ok())
            .collect(),
    );
    println!("{}", solution.solve_1());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            165,
            Solution::new(
                "
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"[1..]
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect()
            )
            .solve_1()
        );
    }
}
