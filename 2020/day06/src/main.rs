use std::io::{BufRead, BufReader};

struct Solution {
    inputs: Vec<String>,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        Self { inputs }
    }
    fn solve_1(&self) -> usize {
        let mut ret = 0;
        let mut d = [false; 26];
        for line in self.inputs.iter().chain([String::new()].iter()) {
            if line.is_empty() {
                for b in d.iter_mut() {
                    if *b {
                        ret += 1;
                    }
                    *b = false;
                }
            } else {
                for &b in line.as_bytes() {
                    d[(b - b'a') as usize] = true;
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
}
