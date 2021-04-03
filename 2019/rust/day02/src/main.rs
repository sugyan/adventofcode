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
        let mut replaced = self.program.clone();
        replaced[1] = 12;
        replaced[2] = 2;
        let mut computer = Intcode::new(&replaced);
        computer.run(None);
        computer.program[0]
    }
    fn part_2(&self) -> i32 {
        for noun in 0..=99 {
            for verb in 0..=99 {
                let mut replaced = self.program.clone();
                replaced[1] = noun;
                replaced[2] = verb;
                let mut computer = Intcode::new(&replaced);
                computer.run(None);
                if computer.program[0] == 19_690_720 {
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
