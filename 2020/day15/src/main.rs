use std::io::{BufRead, BufReader};

struct Solution {
    inputs: Vec<usize>,
}

impl Solution {
    fn new(inputs: Vec<usize>) -> Self {
        Self { inputs }
    }
    fn solve_1(&self) -> usize {
        self.play(2020)
    }
    fn solve_2(&self) -> usize {
        self.play(30_000_000)
    }
    fn play(&self, target: usize) -> usize {
        let mut memory: Vec<usize> = vec![0; target];
        self.inputs
            .iter()
            .enumerate()
            .for_each(|(i, &input)| memory[input] = i + 1);
        let mut prev = self.inputs[self.inputs.len() - 1];
        for i in self.inputs.len()..target {
            let val = match memory[prev] {
                0 => 0,
                j => i - j,
            };
            memory[prev] = i;
            prev = val;
        }
        prev
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
    println!("{}", solution.solve_2());
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

    #[test]
    fn example_2() {
        assert_eq!(175594, Solution::new(vec![0, 3, 6]).solve_2());
        assert_eq!(2578, Solution::new(vec![1, 3, 2]).solve_2());
        assert_eq!(3544142, Solution::new(vec![2, 1, 3]).solve_2());
        assert_eq!(261214, Solution::new(vec![1, 2, 3]).solve_2());
        assert_eq!(6895259, Solution::new(vec![2, 3, 1]).solve_2());
        assert_eq!(18, Solution::new(vec![3, 2, 1]).solve_2());
        assert_eq!(362, Solution::new(vec![3, 1, 2]).solve_2());
    }
}
