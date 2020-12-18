use regex::Regex;
use std::io::{BufRead, BufReader};

struct Solution {
    rules: Vec<(String, [(u32, u32); 2])>,
    ticket: Vec<u32>,
    nearby: Vec<Vec<u32>>,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        let re_field = Regex::new(r"^(.+?): (\d+)\-(\d+) or (\d+)\-(\d+)$").unwrap();
        let mut rules = Vec::new();
        let mut nearby = Vec::new();
        let mut ticket = Vec::new();
        let mut read_nearby = false;
        for line in inputs.iter() {
            if let Some(cap) = re_field.captures(line) {
                if let (Ok(n1), Ok(n2), Ok(n3), Ok(n4)) = (
                    cap[2].parse::<u32>(),
                    cap[3].parse::<u32>(),
                    cap[4].parse::<u32>(),
                    cap[5].parse::<u32>(),
                ) {
                    rules.push((cap[1].to_string(), [(n1, n2), (n3, n4)]));
                }
            }
            if line.starts_with(|c: char| c.is_numeric()) {
                if read_nearby {
                    nearby.push(line.split(',').filter_map(|s| s.parse().ok()).collect())
                } else {
                    ticket.extend(line.split(',').filter_map(|s| s.parse::<u32>().ok()));
                }
            }
            if line.starts_with("nearby") {
                read_nearby = true;
            }
        }
        Self {
            rules,
            ticket,
            nearby,
        }
    }
    fn solve_1(&self) -> u64 {
        self.identify().0
    }
    fn solve_2(&self) -> u64 {
        self.identify()
            .1
            .iter()
            .enumerate()
            .filter(|(_, field)| field.starts_with("departure"))
            .map(|(i, _)| self.ticket[i] as u64)
            .product()
    }
    fn identify(&self) -> (u64, Vec<String>) {
        let max = self
            .rules
            .iter()
            .filter_map(|rule| rule.1.iter().map(|&range| range.1).max())
            .max()
            .unwrap();
        let mut v: Vec<u32> = vec![0; max as usize + 1];
        for (i, rule) in self.rules.iter().enumerate() {
            for &r in rule.1.iter() {
                (r.0..=r.1).for_each(|j| v[j as usize] |= 1 << i);
            }
        }
        let mut candidates = vec![(1 << self.ticket.len()) - 1; self.ticket.len()];
        let mut error_rate = 0;
        for ticket in self.nearby.iter() {
            let mut valid = true;
            for &val in ticket.iter() {
                if val > max || v[val as usize] == 0 {
                    error_rate += val as u64;
                    valid = false
                }
            }
            if valid {
                for (i, &val) in ticket.iter().enumerate() {
                    let n = v[val as usize];
                    candidates[i] &= n;
                }
            }
        }
        let mut fields = vec![String::new(); self.ticket.len()];
        while fields.iter().any(|s| s.is_empty()) {
            if let Some(i) = candidates.iter().position(|&c| c.count_ones() == 1) {
                let n = candidates[i];
                fields[i] += self.rules[n.trailing_zeros() as usize].0.as_str();
                candidates.iter_mut().for_each(|c| *c &= !n);
            }
        }
        (error_rate, fields)
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
            .identify()
            .0
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec!["row", "class", "seat"],
            Solution::new(
                "
class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"[1..]
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect()
            )
            .identify()
            .1
        );
    }
}
