use std::collections::{HashSet, VecDeque};
use std::io::{BufRead, BufReader};

enum Winner {
    Player1,
    Player2,
}

struct Solution {
    decks: [VecDeque<u8>; 2],
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        let mut decks = [VecDeque::new(), VecDeque::new()];
        for (i, lines) in inputs.split(String::is_empty).enumerate() {
            lines
                .iter()
                .skip(1)
                .filter_map(|s| s.parse().ok())
                .for_each(|card| decks[i].push_back(card))
        }
        Self { decks }
    }
    fn part_1(&self) -> u32 {
        let mut decks = self.decks.clone();
        Solution::combat(&mut decks, false);
        decks
            .iter()
            .map(|deck| {
                deck.iter()
                    .rev()
                    .enumerate()
                    .map(|(i, &card)| (i as u32 + 1) * card as u32)
                    .sum::<u32>()
            })
            .sum()
    }
    fn part_2(&self) -> u32 {
        let mut decks = self.decks.clone();
        Solution::combat(&mut decks, true);
        decks
            .iter()
            .map(|deck| {
                deck.iter()
                    .rev()
                    .enumerate()
                    .map(|(i, &card)| (i as u32 + 1) * card as u32)
                    .sum::<u32>()
            })
            .sum()
    }
    fn combat(decks: &mut [VecDeque<u8>; 2], recursive: bool) -> Winner {
        let mut memo = HashSet::new();
        while decks.iter().all(|deck| !deck.is_empty()) {
            if recursive {
                let key = {
                    let mut v = Vec::with_capacity(decks[0].len() + decks[1].len() + 1);
                    v.extend(decks[0].iter());
                    v.push(0);
                    v.extend(decks[1].iter());
                    v
                };
                if memo.contains(&key) {
                    return Winner::Player1;
                }
                memo.insert(key);
            }
            if let (Some(top0), Some(top1)) = (decks[0].pop_front(), decks[1].pop_front()) {
                let winner = if recursive
                    && top0 as usize <= decks[0].len()
                    && top1 as usize <= decks[1].len()
                {
                    let mut new_decks = [
                        decks[0].clone().into_iter().take(top0 as usize).collect(),
                        decks[1].clone().into_iter().take(top1 as usize).collect(),
                    ];
                    Solution::combat(&mut new_decks, recursive)
                } else {
                    if top0 > top1 {
                        Winner::Player1
                    } else {
                        Winner::Player2
                    }
                };
                match winner {
                    Winner::Player1 => {
                        decks[0].push_back(top0);
                        decks[0].push_back(top1);
                    }
                    Winner::Player2 => {
                        decks[1].push_back(top1);
                        decks[1].push_back(top0);
                    }
                }
            }
        }
        if decks[1].is_empty() {
            Winner::Player1
        } else {
            Winner::Player2
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
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_inputs() -> Vec<String> {
        r"
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
10"
        .split('\n')
        .skip(1)
        .map(str::to_string)
        .collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(306, Solution::new(example_inputs()).part_1());
    }

    #[test]
    fn example_2() {
        assert_eq!(291, Solution::new(example_inputs()).part_2());
    }
}
