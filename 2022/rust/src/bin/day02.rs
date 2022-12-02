use aoc2022::Solve;
use std::io::{BufRead, BufReader, Read};

#[derive(Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn shape_score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
    fn outcome(&self, opponent: Self) -> Outcome {
        match (self, opponent) {
            (Self::Rock, Self::Scissors)
            | (Self::Paper, Self::Rock)
            | (Self::Scissors, Self::Paper) => Outcome::Win,
            (Self::Paper, Self::Scissors)
            | (Self::Scissors, Self::Rock)
            | (Self::Rock, Self::Paper) => Outcome::Lose,
            _ => Outcome::Draw,
        }
    }
}

impl From<&str> for Hand {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => unreachable!(),
        }
    }
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }
}

struct Solution {
    hands: Vec<(Hand, Hand)>,
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        Self {
            hands: BufReader::new(r)
                .lines()
                .filter_map(Result::ok)
                .map(|s| {
                    let t = s.split_once(' ').unwrap();
                    (t.0.into(), t.1.into())
                })
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.hands
            .iter()
            .map(|(opponent, hand)| hand.outcome(*opponent).score() + hand.shape_score())
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
A Y
B X
C Z"[1..]
            .as_bytes()
    }

    #[test]
    fn example1() {
        assert_eq!(15, Solution::new(example_input()).part1());
    }
}
