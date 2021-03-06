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
        let mut replaced = self.program.clone();
        replaced[1] = 12;
        replaced[2] = 2;
        let mut computer = Intcode::new(&replaced);
        computer.run(Vec::new());
        *computer.memory.get(&0).unwrap()
    }
    fn part_2(&self) -> i64 {
        for noun in 0..=99 {
            for verb in 0..=99 {
                let mut replaced = self.program.clone();
                replaced[1] = noun;
                replaced[2] = verb;
                let mut computer = Intcode::new(&replaced);
                computer.run(Vec::new());
                if computer.memory.get(&0) == Some(&19_690_720) {
                    return noun * 100 + verb;
                }
            }
        }
        unreachable!()
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
