use std::io::{BufRead, BufReader};

struct Solution {
    groups: Vec<Vec<u32>>,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        let mut groups: Vec<Vec<u32>> = Vec::new();
        let mut v: Vec<u32> = Vec::new();
        for line in inputs.iter().chain([String::new()].iter()) {
            if line.is_empty() {
                groups.push(v.clone());
                v.clear();
            } else {
                v.push(
                    line.as_bytes()
                        .iter()
                        .map(|&b| 1 << (b - b'a') as usize)
                        .fold(0, |acc, x| acc | x),
                );
            }
        }
        Self { groups }
    }
    fn solve_1(&self) -> usize {
        self.groups
            .iter()
            .map(|group| group.iter().fold(0, |acc, &x| acc | x).count_ones() as usize)
            .sum()
    }
    fn solve_2(&self) -> usize {
        self.groups
            .iter()
            .map(|group| {
                group
                    .iter()
                    .fold((1 << 26) - 1, |acc, &x| acc & x)
                    .count_ones() as usize
            })
            .sum()
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
            11,
            Solution::new(
                "
abc

a
b
c

ab
ac

a
a
a
a

b"[1..]
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
            6,
            Solution::new(
                "
abc

a
b
c

ab
ac

a
a
a
a

b"[1..]
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect()
            )
            .solve_2()
        );
    }
}
