use aoc2023::Solve;
use itertools::Itertools;
use std::cmp::Ordering;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Card {
    Number(u8),
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn strength_value(&self) -> usize {
        match self {
            Self::Number(n) => *n as usize - 2,
            Self::T => 8,
            Self::J => 9,
            Self::Q => 10,
            Self::K => 11,
            Self::A => 12,
        }
    }
}

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'T' => Ok(Self::T),
            'J' => Ok(Self::J),
            'Q' => Ok(Self::Q),
            'K' => Ok(Self::K),
            'A' => Ok(Self::A),
            '2'..='9' => Ok(Self::Number(c as u8 - b'0')),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
}

impl Hand {
    fn type_value(&self) -> u32 {
        let mut counts = [0; 13];
        for c in &self.cards {
            counts[c.strength_value()] += 1;
        }
        match counts.iter().sorted().rev().take(3).collect_tuple() {
            Some((5, 0, 0)) => 6,
            Some((4, 1, 0)) => 5,
            Some((3, 2, 0)) => 4,
            Some((3, 1, 1)) => 3,
            Some((2, 2, 1)) => 2,
            Some((2, 1, 1)) => 1,
            _ => 0,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.type_value().cmp(&other.type_value()) {
            Ordering::Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .map(|(a, b)| a.strength_value().cmp(&b.strength_value()))
                .find(|&o| o != Ordering::Equal)
                .unwrap_or(Ordering::Equal),
            o => o,
        }
    }
}

#[derive(Debug)]
enum ParseHandError {
    InvalidLength,
    InvalidCard,
}

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 5 {
            return Err(ParseHandError::InvalidLength);
        }
        let mut cards = [Card::Number(0); 5];
        for (i, c) in s.chars().enumerate() {
            cards[i] = c.try_into().map_err(|_| ParseHandError::InvalidCard)?;
        }
        Ok(Self { cards })
    }
}

struct Solution {
    list: Vec<(Hand, u32)>,
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        Self {
            list: BufReader::new(r)
                .lines()
                .map_while(Result::ok)
                .map(|line| {
                    let (hand, bid) = line.split_once(' ').expect("invalid line");
                    (
                        hand.parse().expect("invalid hand"),
                        bid.parse().expect("invalid bid"),
                    )
                })
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.list
            .iter()
            .sorted()
            .enumerate()
            .map(|(i, (_, bid))| (i as u32 + 1) * bid)
            .sum()
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
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(Solution::new(example_input()).part1(), 6440);
    }
}
