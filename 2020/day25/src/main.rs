// use std::collections::HashMap;
use std::io::{BufRead, BufReader};

struct Solution {
    card_key: u64,
    door_key: u64,
}

impl Solution {
    fn new(inputs: Vec<u64>) -> Self {
        Self {
            card_key: inputs[0],
            door_key: inputs[1],
        }
    }
    fn solve_1(&self) -> u64 {
        for subject_number in 2..30 {
            if let Some(card_loop_size) = self.loop_size(subject_number, self.card_key) {
                if let Some(door_loop_size) = self.loop_size(subject_number, self.door_key) {
                    if card_loop_size != door_loop_size {
                        return (0..card_loop_size)
                            .fold(1, |acc, _| (acc * self.door_key) % 20201227);
                    }
                }
            }
        }
        0
    }
    fn loop_size(&self, subject_number: u64, target: u64) -> Option<usize> {
        let mut value = 1;
        for i in 0..20201227 {
            if value == target {
                return Some(i);
            }
            value = (value * subject_number) % 20201227;
        }
        None
    }
}

fn main() {
    let solution = Solution::new(
        BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(|line| line.ok())
            .filter_map(|s| s.parse().ok())
            .collect(),
    );
    println!("{}", solution.solve_1());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(14897079, Solution::new(vec![5764801, 17807724]).solve_1());
    }
}
