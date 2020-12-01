use std::io::{BufRead, BufReader};

struct Solution {
    inputs: Vec<i32>,
}

impl Solution {
    fn new(inputs: Vec<i32>) -> Self {
        Self { inputs }
    }
    fn solve(&self) -> i32 {
        for i in 0..self.inputs.len() - 1 {
            for j in i..self.inputs.len() {
                if self.inputs[i] + self.inputs[j] == 2020 {
                    return self.inputs[i] * self.inputs[j];
                }
            }
        }
        0
    }
}

fn main() {
    let inputs: Vec<i32> = BufReader::new(std::io::stdin().lock())
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|s| s.parse().ok())
        .collect();
    println!("{}", Solution::new(inputs).solve());
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            514579,
            Solution::new(vec![1721, 979, 366, 299, 675, 1456]).solve()
        );
    }
}
