use intcode::Intcode;
use std::io::{BufRead, BufReader};

struct Solution {
    program: Vec<i64>,
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
    fn part_1(&self) -> i64 {
        let mut computer = Intcode::new(&self.program);
        loop {
            if let intcode::Result::Output(n) = computer.run(vec![1]) {
                if n != 0 {
                    return n;
                }
            }
        }
    }
    fn part_2(&self) -> i64 {
        let mut computer = Intcode::new(&self.program);
        match computer.run(vec![5]) {
            intcode::Result::Output(n) => n,
            _ => unreachable!(),
        }
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
