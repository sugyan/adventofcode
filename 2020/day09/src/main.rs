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
        'numbers: for i in self.preamble..self.inputs.len() {
            for j in i - self.preamble..i - 1 {
                for k in j + 1..i {
                    if self.inputs[j] + self.inputs[k] == self.inputs[i] {
                        continue 'numbers;
                    }
                }
            }
            return self.inputs[i];
        }
        0
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
}
