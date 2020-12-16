use regex::Regex;
use std::io::{BufRead, BufReader};

struct Solution {
    rules: Vec<[(u32, u32); 2]>,
    nearby: Vec<Vec<u32>>,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        let re_field = Regex::new(r"^.*: (\d+)\-(\d+) or (\d+)\-(\d+)$").unwrap();
        let mut rules = Vec::new();
        let mut nearby = Vec::new();
        let mut read_nearby = false;
        for line in inputs.iter() {
            if let Some(cap) = re_field.captures(line) {
                if let (Ok(n1), Ok(n2), Ok(n3), Ok(n4)) = (
                    cap[1].parse::<u32>(),
                    cap[2].parse::<u32>(),
                    cap[3].parse::<u32>(),
                    cap[4].parse::<u32>(),
                ) {
                    rules.push([(n1, n2), (n3, n4)]);
                }
            }
            if read_nearby {
                nearby.push(line.split(',').filter_map(|s| s.parse().ok()).collect())
            }
            if line.starts_with("nearby") {
                read_nearby = true;
            }
        }
        Self { rules, nearby }
    }
    fn solve_1(&self) -> u32 {
        let max = self
            .rules
            .iter()
            .filter_map(|rule| rule.iter().map(|&range| range.1).max())
            .max()
            .unwrap();
        let mut v: Vec<bool> = vec![false; max as usize + 1];
        for &rule in self.rules.iter() {
            for &r in rule.iter() {
                (r.0..=r.1).for_each(|i| v[i as usize] = true);
            }
        }
        let mut ret = 0;
        for ticket in self.nearby.iter() {
            for &val in ticket.iter() {
                if val > max || !v[val as usize] {
                    ret += val;
                }
            }
        }
        ret
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
            71,
            Solution::new(
                "
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"[1..]
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect()
            )
            .solve_1()
        );
    }
}
