use std::collections::VecDeque;
use std::io::{BufRead, BufReader};

struct Solution {
    inputs: Vec<String>,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        Self { inputs }
    }
    fn solve_1(&self) -> u64 {
        let mut player1 = true;
        let mut decks = (VecDeque::new(), VecDeque::new());
        for input in self.inputs.iter().filter(|&s| !s.is_empty()) {
            if input.starts_with("Player 2") {
                player1 = false;
            }
            if let Ok(card) = input.parse() {
                if player1 {
                    decks.0.push_back(card);
                } else {
                    decks.1.push_back(card)
                }
            }
        }
        while !decks.0.is_empty() && !decks.1.is_empty() {
            if let (Some(top1), Some(top2)) = (decks.0.pop_front(), decks.1.pop_front()) {
                if top1 > top2 {
                    decks.0.push_back(top1);
                    decks.0.push_back(top2);
                } else {
                    decks.1.push_back(top2);
                    decks.1.push_back(top1);
                }
            }
        }
        if decks.0.is_empty() {
            decks
                .1
                .iter()
                .rev()
                .enumerate()
                .map(|(i, card)| (i as u64 + 1) * card)
                .sum()
        } else {
            decks
                .0
                .iter()
                .rev()
                .enumerate()
                .map(|(i, card)| (i as u64 + 1) * card)
                .sum()
        }
    }
}

fn main() {
    let solution = Solution::new(
        BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(|line| line.ok())
            .collect(),
    );
    println!("{}", solution.solve_1());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            306,
            Solution::new(
                "
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"[1..]
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect()
            )
            .solve_1()
        );
    }
}
