use std::collections::VecDeque;
use std::io::{BufRead, BufReader};

struct Solution {
    inputs: Vec<u64>,
    preamble: usize,
}

impl Solution {
    fn new(inputs: Vec<u64>, preamble: usize) -> Self {
        Self { inputs, preamble }
    }
    fn solve_1(&self) -> u64 {
        let mut vd: VecDeque<&u64> = self.inputs.iter().take(self.preamble).collect();
        let has_pair = |vd: &VecDeque<&u64>, target: u64| -> bool {
            for &n1 in vd.iter() {
                for &n2 in vd.iter() {
                    if *n1 != *n2 && *n1 + *n2 == target {
                        return true;
                    }
                }
            }
            false
        };
        for i in self.preamble..self.inputs.len() {
            if !has_pair(&vd, self.inputs[i]) {
                return self.inputs[i];
            }
            vd.pop_front();
            vd.push_back(&self.inputs[i]);
        }
        0
    }
    fn solve_2(&self) -> u64 {
        let target = self.solve_1();
        let (mut l, mut r) = (0, 0);
        let mut sum = self.inputs[0];
        while sum != target {
            if sum < target {
                r += 1;
                sum += self.inputs[r];
            } else {
                sum -= self.inputs[l];
                l += 1;
            }
        }
        let (mut min, mut max) = (std::u64::MAX, std::u64::MIN);
        for i in l..=r {
            min = std::cmp::min(min, self.inputs[i]);
            max = std::cmp::max(max, self.inputs[i]);
        }
        min + max
    }
}

fn main() {
    let solution = Solution::new(
        BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(|line| line.ok())
            .filter_map(|s| s.parse::<u64>().ok())
            .collect(),
        25,
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
            127,
            Solution::new(
                vec![
                    35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                    309, 576,
                ],
                5
            )
            .solve_1()
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            62,
            Solution::new(
                vec![
                    35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                    309, 576,
                ],
                5
            )
            .solve_2()
        );
    }
}
