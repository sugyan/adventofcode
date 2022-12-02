use aoc2022::Solve;
use std::io::{BufRead, BufReader, Read};

#[derive(Clone, Copy, PartialEq, Eq)]
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
    fn do_round(&self, opponent: Self) -> Outcome {
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
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            _ => unreachable!(),
        }
    }
}

enum Choose {
    X,
    Y,
    Z,
}

impl Choose {
    fn hand(&self, strategy: Strategy, opponent: &Hand) -> Hand {
        match strategy {
            Strategy::Part1 => match self {
                Self::X => Hand::Rock,
                Self::Y => Hand::Paper,
                Self::Z => Hand::Scissors,
            },
            Strategy::Part2 => match self {
                Self::X => match opponent {
                    Hand::Rock => Hand::Scissors,
                    Hand::Paper => Hand::Rock,
                    Hand::Scissors => Hand::Paper,
                },
                Self::Y => *opponent,
                Self::Z => match opponent {
                    Hand::Rock => Hand::Paper,
                    Hand::Paper => Hand::Scissors,
                    Hand::Scissors => Hand::Rock,
                },
            },
        }
    }
}

impl From<&str> for Choose {
    fn from(s: &str) -> Self {
        match s {
            "X" => Self::X,
            "Y" => Self::Y,
            "Z" => Self::Z,
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

#[derive(Clone, Copy)]
enum Strategy {
    Part1,
    Part2,
}

struct Solution {
    strategy_guide: Vec<(Hand, Choose)>,
}

impl Solution {
    fn total_score(&self, strategy: Strategy) -> u32 {
        self.strategy_guide
            .iter()
            .map(|(opponent, choose)| {
                let hand = choose.hand(strategy, opponent);
                hand.do_round(*opponent).score() + hand.shape_score()
            })
            .sum()
    }
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        Self {
            strategy_guide: BufReader::new(r)
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
        self.total_score(Strategy::Part1)
    }
    fn part2(&self) -> Self::Answer2 {
        self.total_score(Strategy::Part2)
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

    #[test]
    fn example2() {
        assert_eq!(12, Solution::new(example_input()).part2());
    }
}
