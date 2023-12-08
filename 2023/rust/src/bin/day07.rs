use aoc2023::Solve;
use itertools::Itertools;
use std::array;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

#[derive(Clone, Copy)]
enum Card {
    Number(u8),
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn strength_value(&self, joker: bool) -> usize {
        match self {
            Self::Number(n) => *n as usize,
            Self::T => 10,
            Self::J if joker => 0,
            Self::J => 11,
            Self::Q => 12,
            Self::K => 13,
            Self::A => 14,
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

struct Hand {
    cards: [Card; 5],
}

impl Hand {
    fn values(&self, joker: bool) -> (u8, [usize; 5]) {
        (
            self.type_value(joker),
            array::from_fn(|i| self.cards[i].strength_value(joker)),
        )
    }
    fn type_value(&self, joker: bool) -> u8 {
        let mut counts = [0; 15];
        for c in &self.cards {
            counts[c.strength_value(joker)] += 1;
        }
        let mut v = counts[1..]
            .iter()
            .sorted()
            .rev()
            .take(2)
            .cloned()
            .collect_vec();
        if joker {
            v[0] += counts[0];
        }
        match v.as_slice() {
            [5, 0] => 6,
            [4, 1] => 5,
            [3, 2] => 4,
            [3, 1] => 3,
            [2, 2] => 2,
            [2, 1] => 1,
            _ => 0,
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
        let v: Vec<_> = s
            .chars()
            .map(|c| c.try_into().map_err(|_| ParseHandError::InvalidCard))
            .try_collect()?;
        Ok(Self {
            cards: array::from_fn(|i| v[i]),
        })
    }
}

struct Solution {
    list: Vec<(Hand, u32)>,
}

impl Solution {
    fn total_winnings(&self, joker: bool) -> u32 {
        (1..)
            .zip(
                self.list
                    .iter()
                    .sorted_by_cached_key(|(hand, _)| hand.values(joker)),
            )
            .map(|(i, (_, bid))| i * bid)
            .sum()
    }
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
                    line.split_once(' ')
                        .map(|(hand, bid)| {
                            (
                                hand.parse().expect("invalid hand"),
                                bid.parse().expect("invalid bid"),
                            )
                        })
                        .expect("invalid line")
                })
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.total_winnings(false)
    }
    fn part2(&self) -> Self::Answer2 {
        self.total_winnings(true)
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

    #[test]
    fn part2() {
        assert_eq!(Solution::new(example_input()).part2(), 5905);
    }
}
