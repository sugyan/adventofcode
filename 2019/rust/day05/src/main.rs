use intcode::Intcode;
use std::io::{BufRead, BufReader};

struct Solution {
    program: Vec<i32>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        Self {
            program: inputs[0]
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect(),
        }
    }
    fn part_1(&self) -> i32 {
        let mut computer = Intcode::new(&self.program);
        computer.run(vec![1]).unwrap()
    }
    fn part_2(&self) -> i32 {
        let mut computer = Intcode::new(&self.program);
        computer.run(vec![5]).unwrap()
    }
}

fn main() {
    let solution = Solution::new(
        &BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(Result::ok)
            .collect::<Vec<_>>(),
    );
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}
