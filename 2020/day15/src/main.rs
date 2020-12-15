use std::collections::HashMap;
use std::io::{BufRead, BufReader};

struct Solution {
    inputs: Vec<usize>,
}

impl Solution {
    fn new(inputs: Vec<usize>) -> Self {
        Self { inputs }
    }
    fn solve_1(&self) -> usize {
        let mut numbers: Vec<usize> = Vec::with_capacity(2020);
        let mut memory: HashMap<usize, usize> = HashMap::new();
        let mut prev = 0;
        for (i, &input) in self.inputs.iter().enumerate() {
            if i > 0 {
                memory.insert(prev, i - 1);
            }
            numbers.push(input);
            prev = input;
        }
        for i in numbers.len()..2020 {
            let next = if let Some(&j) = memory.get(&prev) {
                i - j - 1
            } else {
                0
            };
            memory.insert(prev, i - 1);
            numbers.push(next);
            prev = next;
        }
        numbers[2019]
    }
}

fn main() {
    let solution = Solution::new(
        BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(|line| line.ok())
            .map(|line| {
                line.split(',')
                    .filter_map(|s| s.parse().ok())
                    .collect::<Vec<usize>>()
            })
            .flatten()
            .collect(),
    );
    println!("{}", solution.solve_1());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(436, Solution::new(vec![0, 3, 6]).solve_1());
        assert_eq!(1, Solution::new(vec![1, 3, 2]).solve_1());
        assert_eq!(10, Solution::new(vec![2, 1, 3]).solve_1());
        assert_eq!(27, Solution::new(vec![1, 2, 3]).solve_1());
        assert_eq!(78, Solution::new(vec![2, 3, 1]).solve_1());
        assert_eq!(438, Solution::new(vec![3, 2, 1]).solve_1());
        assert_eq!(1836, Solution::new(vec![3, 1, 2]).solve_1());
    }
}
