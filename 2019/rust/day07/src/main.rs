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
        Self::permutations(&(0..5).collect::<Vec<_>>())
            .iter()
            .map(|p| {
                p.iter().fold(0, |acc, &phase| {
                    match Intcode::new(&self.program).run(vec![phase, acc]) {
                        intcode::Result::Output(out) => out,
                        _ => unreachable!(),
                    }
                })
            })
            .max()
            .unwrap()
    }
    fn part_2(&self) -> i32 {
        Self::permutations(&(5..10).collect::<Vec<_>>())
            .iter()
            .filter_map(|p| {
                let mut amplifiers = p
                    .iter()
                    .map(|&phase| {
                        let mut amplifier = Intcode::new(&self.program);
                        amplifier.run(vec![phase]);
                        amplifier
                    })
                    .collect::<Vec<_>>();
                std::iter::successors(Some(0), |&output| {
                    amplifiers
                        .iter_mut()
                        .try_fold(output, |acc, amp| match amp.run(vec![acc]) {
                            intcode::Result::Output(n) => Some(n),
                            _ => None,
                        })
                })
                .last()
            })
            .max()
            .unwrap()
    }
    fn permutations(phases: &[i32]) -> Vec<Vec<i32>> {
        fn backtrack(phases: &[i32], v: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>) {
            if v.len() == phases.len() {
                ret.push(v.clone());
            } else {
                for p in phases {
                    if !v.contains(p) {
                        v.push(*p);
                        backtrack(phases, v, ret);
                        v.pop();
                    }
                }
            }
        }
        let mut v = Vec::new();
        let mut ret = Vec::new();
        backtrack(&phases, &mut v, &mut ret);
        ret
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            43210,
            Solution::new(&[String::from(
                "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"
            )])
            .part_1()
        );
        assert_eq!(
            54321,
            Solution::new(&[String::from(
                "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"
            )])
            .part_1()
        );
        assert_eq!(
            65210,
            Solution::new(&[String::from(
                "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"
            )])
            .part_1()
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            139_629_729,
            Solution::new(&[String::from(
                "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
            )])
            .part_2()
        );
        assert_eq!(
            18216,
            Solution::new(&[String::from(
                "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
            )])
            .part_2()
        );
    }
}
