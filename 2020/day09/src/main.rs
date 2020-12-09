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
        for &n in self.inputs.iter().skip(l).take(r - l + 1) {
            min = std::cmp::min(min, n);
            max = std::cmp::max(max, n);
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
