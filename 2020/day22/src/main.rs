use std::collections::{HashSet, VecDeque};
use std::io::{BufRead, BufReader};

struct Solution {
    decks: [VecDeque<u32>; 2],
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        let mut player1 = true;
        let mut decks = [VecDeque::new(), VecDeque::new()];
        for input in inputs.iter().filter(|&s| !s.is_empty()) {
            if input.starts_with("Player 2") {
                player1 = false;
            }
            if let Ok(card) = input.parse() {
                if player1 {
                    decks[0].push_back(card);
                } else {
                    decks[1].push_back(card)
                }
            }
        }
        Self { decks }
    }
    fn solve_1(&self) -> u32 {
        let mut decks = self.decks.clone();
        Solution::combat(&mut decks, false);
        decks
            .iter()
            .map(|deck| {
                deck.iter()
                    .rev()
                    .enumerate()
                    .map(|(i, &card)| (i as u32 + 1) * card)
                    .sum::<u32>()
            })
            .sum()
    }
    fn solve_2(&self) -> u32 {
        let mut decks = self.decks.clone();
        Solution::combat(&mut decks, true);
        decks
            .iter()
            .map(|deck| {
                deck.iter()
                    .rev()
                    .enumerate()
                    .map(|(i, &card)| (i as u32 + 1) * card)
                    .sum::<u32>()
            })
            .sum()
    }
    fn combat(decks: &mut [VecDeque<u32>; 2], recursive: bool) -> bool {
        let mut memo = HashSet::new();
        while decks.iter().all(|deck| !deck.is_empty()) {
            if recursive {
                let key = format!("{:?}", decks);
                if memo.contains(&key) {
                    return true;
                }
                memo.insert(key);
            }
            if let (Some(top0), Some(top1)) = (decks[0].pop_front(), decks[1].pop_front()) {
                let player1_wins = if recursive
                    && top0 as usize <= decks[0].len()
                    && top1 as usize <= decks[1].len()
                {
                    let mut new_decks = [
                        decks[0].clone().into_iter().take(top0 as usize).collect(),
                        decks[1].clone().into_iter().take(top1 as usize).collect(),
                    ];
                    Solution::combat(&mut new_decks, recursive)
                } else {
                    top0 > top1
                };
                if player1_wins {
                    decks[0].push_back(top0);
                    decks[0].push_back(top1);
                } else {
                    decks[1].push_back(top1);
                    decks[1].push_back(top0);
                }
            }
        }
        decks[1].is_empty()
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
    println!("{}", solution.solve_2());
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

    #[test]
    fn example_2() {
        assert_eq!(
            291,
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
            .solve_2()
        );
    }
}
