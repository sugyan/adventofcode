use std::io::{BufRead, BufReader};

struct Solution {
    inputs: Vec<u64>,
}

impl Solution {
    fn new(inputs: Vec<u64>) -> Self {
        let mut inputs = inputs;
        inputs.push(0);
        inputs.sort_unstable();
        Self { inputs }
    }
    fn solve_1(&self) -> u64 {
        let (mut diff1, mut diff3) = (0, 0);
        for i in 1..self.inputs.len() {
            match self.inputs[i] - self.inputs[i - 1] {
                1 => diff1 += 1,
                3 => diff3 += 1,
                _ => {}
            }
        }
        diff1 * (diff3 + 1)
    }
    fn solve_2(&self) -> u64 {
        let mut v: Vec<u64> = vec![0; self.inputs.len()];
        v[self.inputs.len() - 1] = 1;
        for i in (0..v.len()).rev().skip(1) {
            for j in 1..=3 {
                if i + j < v.len() && self.inputs[i + j] - self.inputs[i] <= 3 {
                    v[i] += v[i + j];
                }
            }
        }
        v[0]
    }
}

fn main() {
    let solution = Solution::new(
        BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(|line| line.ok())
            .filter_map(|s| s.parse::<u64>().ok())
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
            35,
            Solution::new(vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]).solve_1()
        );
        assert_eq!(
            220,
            Solution::new(vec![
                28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25,
                35, 8, 17, 7, 9, 4, 2, 34, 10, 3
            ])
            .solve_1()
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            8,
            Solution::new(vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]).solve_2()
        );
        assert_eq!(
            19208,
            Solution::new(vec![
                28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25,
                35, 8, 17, 7, 9, 4, 2, 34, 10, 3
            ])
            .solve_2()
        );
    }
}
