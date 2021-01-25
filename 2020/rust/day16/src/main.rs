use std::io::{BufRead, BufReader};

struct Solution {
    rules: Vec<(String, Vec<(u32, u32)>)>,
    ticket: Vec<u32>,
    nearby: Vec<Vec<u32>>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        let mut rules = Vec::new();
        let mut nearby = Vec::new();
        let mut ticket = Vec::new();
        let mut read_nearby = false;
        for line in inputs.iter().filter(|&s| !s.is_empty()) {
            if line.starts_with(char::is_numeric) {
                let values = line.split(',').filter_map(|s| s.parse().ok()).collect();
                if read_nearby {
                    nearby.push(values);
                } else {
                    ticket.extend(values);
                }
            } else if line.ends_with(char::is_numeric) {
                let kv = line.split(": ").collect::<Vec<_>>();
                let ranges = kv[1]
                    .split(" or ")
                    .filter_map(|range| {
                        range
                            .split('-')
                            .map(|s| s.parse::<u32>().ok())
                            .collect::<Option<Vec<_>>>()
                            .map(|minmax| (minmax[0], minmax[1]))
                    })
                    .collect();
                rules.push((kv[0].to_string(), ranges));
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
    fn part_1(&self) -> u32 {
        self.identify().0
    }
    fn part_2(&self) -> u64 {
        self.identify()
            .1
            .iter()
            .enumerate()
            .filter_map(|(i, field)| {
                if field.starts_with("departure") {
                    Some(u64::from(self.ticket[i]))
                } else {
                    None
                }
            })
            .product()
    }
    fn identify(&self) -> (u32, Vec<String>) {
        let max = self
            .rules
            .iter()
            .filter_map(|rule| rule.1.iter().map(|&range| range.1).max())
            .max()
            .unwrap();
        let mut v = vec![0_u32; max as usize + 1];
        for (i, rule) in self.rules.iter().enumerate() {
            for &r in &rule.1 {
                (r.0..=r.1).for_each(|j| v[j as usize] |= 1 << i);
            }
        }
        let mut candidates = vec![(1 << self.ticket.len()) - 1; self.ticket.len()];
        let mut error_rate = 0;
        for ticket in &self.nearby {
            let mut valid = true;
            for &val in ticket.iter() {
                if val > max || v[val as usize] == 0 {
                    error_rate += val;
                    valid = false
                }
            }
            if valid {
                for (i, &val) in ticket.iter().enumerate() {
                    candidates[i] &= v[val as usize];
                }
            }
        }
        let mut fields = vec![String::new(); self.ticket.len()];
        for _ in 0..self.ticket.len() {
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
        &BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(Result::ok)
            .collect::<Vec<_>>(),
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
            71,
            Solution::new(
                &r"
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"
                    .split('\n')
                    .skip(1)
                    .map(str::to_string)
                    .collect::<Vec<_>>()
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
                &r"
class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"
                    .split('\n')
                    .skip(1)
                    .map(str::to_string)
                    .collect::<Vec<_>>()
            )
            .identify()
            .1
        );
    }
}
