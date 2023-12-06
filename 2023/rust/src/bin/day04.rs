use aoc2023::Solve;
use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

struct Card {
    winnings: Vec<u32>,
    haves: HashSet<u32>,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (w, h) = s.split(": ").nth(1).unwrap().split_once(" | ").unwrap();
        let winnings = w
            .split_ascii_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        let haves = h
            .split_ascii_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        Ok(Self { winnings, haves })
    }
}

struct Solution {
    matches: Vec<usize>,
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        Self {
            matches: BufReader::new(r)
                .lines()
                .map_while(Result::ok)
                .filter_map(|s| s.parse::<Card>().ok())
                .map(|card| {
                    card.winnings
                        .iter()
                        .filter(|n| card.haves.contains(n))
                        .count()
                })
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.matches.iter().map(|n| (1 << n) >> 1).sum()
    }
    fn part2(&self) -> Self::Answer2 {
        let mut v = vec![1; self.matches.len()];
        for (i, n) in self.matches.iter().enumerate() {
            for j in i + 1..=(i + *n).min(v.len() - 1) {
                v[j] += v[i];
            }
        }
        v.iter().sum()
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
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(Solution::new(example_input()).part1(), 13);
    }

    #[test]
    fn part2() {
        assert_eq!(Solution::new(example_input()).part2(), 30);
    }
}
