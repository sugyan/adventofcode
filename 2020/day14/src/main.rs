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
    fn solve_1(&self) -> u64 {
        let mut mask: (u64, u64) = ((1 << 36) - 1, 0);
        let mut mem: HashMap<u64, u64> = HashMap::new();
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
    fn solve_2(&self) -> u64 {
        let mut mem: HashMap<u64, u64> = HashMap::new();
        let mut mask = "";
        for input in self.inputs.iter() {
            if let Some(m) = input.strip_prefix("mask = ") {
                mask = m;
            } else if let Some(cap) = self.re.captures(input) {
                if let (Ok(address), Ok(value)) = (cap[1].parse::<u64>(), cap[2].parse::<u64>()) {
                    let mut vd: VecDeque<u64> = VecDeque::new();
                    vd.push_back(address);
                    for (i, c) in mask.chars().rev().enumerate() {
                        match c {
                            '1' => vd.iter_mut().for_each(|v| *v |= 1 << i),
                            'X' => {
                                for _ in 0..vd.len() {
                                    if let Some(front) = vd.pop_front() {
                                        vd.push_back(front | 1 << i);
                                        vd.push_back(front & !(1 << i));
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    for &address in vd.iter() {
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
    println!("{}", solution.solve_1());
    println!("{}", solution.solve_2());
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

    #[test]
    fn example_2() {
        assert_eq!(
            208,
            Solution::new(
                "
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
"[1..]
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect()
            )
            .solve_2()
        );
    }
}
