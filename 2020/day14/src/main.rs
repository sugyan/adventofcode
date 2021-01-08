use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::io::{BufRead, BufReader};

struct Solution {
    inputs: Vec<String>,
    re: Regex,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        Self {
            inputs,
            re: Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap(),
        }
    }
    fn part_1(&self) -> u64 {
        let mut mask = ((1 << 36) - 1, 0);
        let mut mem = HashMap::new();
        for input in self.inputs.iter() {
            if let Some(m) = input.strip_prefix("mask = ") {
                mask = ((1 << 36) - 1, 0);
                for (i, c) in m.chars().rev().enumerate() {
                    match c {
                        '0' => mask.0 &= !(1 << i),
                        '1' => mask.1 |= 1 << i,
                        _ => {}
                    }
                }
            } else if let Some(cap) = self.re.captures(input) {
                if let (Ok(address), Ok(value)) = (cap[1].parse::<u64>(), cap[2].parse::<u64>()) {
                    mem.insert(address, value & mask.0 | mask.1);
                }
            }
        }
        mem.values().sum()
    }
    fn part_2(&self) -> u64 {
        let mut mem = HashMap::new();
        let mut masks = (Vec::new(), 0);
        for input in self.inputs.iter() {
            if let Some(m) = input.strip_prefix("mask = ") {
                masks.0.clear();
                masks.1 = 0;
                for (i, c) in m.chars().rev().enumerate() {
                    match c {
                        '1' => masks.1 |= 1 << i,
                        'X' => masks.0.push(i),
                        _ => {}
                    }
                }
            } else if let Some(cap) = self.re.captures(input) {
                if let (Ok(address), Ok(value)) = (cap[1].parse::<u64>(), cap[2].parse::<u64>()) {
                    let mut addresses = VecDeque::new();
                    addresses.push_back(address | masks.1);
                    for &i in masks.0.iter() {
                        for _ in 0..addresses.len() {
                            if let Some(front) = addresses.pop_front() {
                                addresses.push_back(front | 1 << i);
                                addresses.push_back(front & !(1 << i));
                            }
                        }
                    }
                    for &address in addresses.iter() {
                        mem.insert(address, value);
                    }
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
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            165,
            Solution::new(
                r"
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"
                    .split('\n')
                    .skip(1)
                    .map(str::to_string)
                    .collect()
            )
            .part_1()
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            208,
            Solution::new(
                r"
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
"
                .split('\n')
                .skip(1)
                .map(str::to_string)
                .collect()
            )
            .part_2()
        );
    }
}
