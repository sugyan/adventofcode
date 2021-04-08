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
        Self::permutations(5)
            .iter()
            .map(|p| {
                (0..p.len())
                    .map(|_| Intcode::new(&self.program))
                    .collect::<Vec<_>>()
                    .iter_mut()
                    .enumerate()
                    .fold(0, |acc, (i, amp)| {
                        amp.run(vec![p[i], acc]).expect("output value")
                    })
            })
            .max()
            .unwrap()
    }
    fn part_2(&self) -> u32 {
        unimplemented!()
    }
    fn permutations(size: i32) -> Vec<Vec<i32>> {
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
        let phases = (0..size as i32).collect::<Vec<_>>();
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
    }

    #[test]
    fn example_2() {
        assert_eq!(
            54321,
            Solution::new(&[String::from(
                "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"
            )])
            .part_1()
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            65210,
            Solution::new(&[String::from(
                "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"
            )])
            .part_1()
        );
    }
}
