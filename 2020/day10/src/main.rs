use std::io::{BufRead, BufReader};

struct Solution {
    inputs: Vec<i32>,
}

impl Solution {
    fn new(inputs: Vec<i32>) -> Self {
        Self { inputs }
    }
    fn solve_1(&self) -> i32 {
        let mut inputs = self.inputs.clone();
        inputs.push(0);
        inputs.sort_unstable();
        let (mut diff1, mut diff3) = (0, 0);
        for i in 1..inputs.len() {
            match inputs[i] - inputs[i - 1] {
                1 => diff1 += 1,
                3 => diff3 += 1,
                _ => {}
            }
        }
        diff1 * (diff3 + 1)
    }
}

fn main() {
    let solution = Solution::new(
        BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(|line| line.ok())
            .filter_map(|s| s.parse::<i32>().ok())
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
            35,
            Solution::new(vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]).solve_1()
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            220,
            Solution::new(vec![
                28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25,
                35, 8, 17, 7, 9, 4, 2, 34, 10, 3
            ])
            .solve_1()
        );
    }
}