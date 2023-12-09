use aoc2023::Solve;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    histories: Vec<Vec<i32>>,
}

impl Solve for Solution {
    type Answer1 = i32;
    type Answer2 = i32;

    fn new(r: impl Read) -> Self {
        Self {
            histories: BufReader::new(r)
                .lines()
                .map_while(Result::ok)
                .map(|line| {
                    line.split_ascii_whitespace()
                        .map(|s| s.parse().expect("should be valid integer"))
                        .collect()
                })
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let mut answer = 0;
        for history in &self.histories {
            let mut stack = vec![history.clone()];
            while let Some(last) = stack.last() {
                if last.iter().all(|h| *h == 0) {
                    break;
                }
                stack.push(last.windows(2).map(|w| w[1] - w[0]).collect());
            }
            stack.last_mut().expect("should have last").push(0);
            let mut carry = 0;
            while let Some(pop) = stack.pop() {
                if let Some(last) = stack.last_mut() {
                    carry += last[last.len() - 1];
                    last.push(carry);
                }
            }
            answer += carry;
        }
        answer
    }
    fn part2(&self) -> Self::Answer2 {
        todo!()
    }
}

fn main() {
    let solution = Solution::new(std::io::stdin().lock());
    println!("Part 1: {}", solution.part1());
    println!("Part 2: {}", solution.part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static [u8] {
        r"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(Solution::new(example_input()).part1(), 114);
    }
}
