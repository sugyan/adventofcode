use aoc2024::{Solve, run};
use std::{
    io::{BufRead, BufReader, Read},
    str::FromStr,
};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
    #[error("invalid input")]
    InvalidInput,
}

#[derive(Debug)]
struct Position {
    x: i64,
    y: i64,
}

impl FromStr for Position {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(": ")
            .and_then(|(_, s)| s.split_once(", "))
            .ok_or(Error::InvalidInput)
            .and_then(|(x, y)| {
                Ok(Self {
                    x: x[2..].parse()?,
                    y: y[2..].parse()?,
                })
            })
    }
}

#[derive(Debug)]
struct Machine {
    a: Position,
    b: Position,
    prize: Position,
}

impl Machine {
    fn tokens(&self, offset: i64) -> Option<i64> {
        let y_num = self.a.x * (self.prize.y + offset) - self.a.y * (self.prize.x + offset);
        let y_den = self.a.x * self.b.y - self.a.y * self.b.x;
        if y_num % y_den != 0 {
            return None;
        }
        let b = y_num / y_den;
        let x_num = self.prize.x + offset - self.b.x * b;
        let x_den = self.a.x;
        if x_num % x_den != 0 {
            return None;
        }
        let a = x_num / x_den;
        Some(a * 3 + b)
    }
}

impl TryFrom<&[String]> for Machine {
    type Error = Error;

    fn try_from(value: &[String]) -> Result<Self, Self::Error> {
        Ok(Self {
            #[allow(clippy::get_first)]
            a: value.get(0).ok_or(Error::InvalidInput)?.parse()?,
            b: value.get(1).ok_or(Error::InvalidInput)?.parse()?,
            prize: value.get(2).ok_or(Error::InvalidInput)?.parse()?,
        })
    }
}

struct Solution {
    machines: Vec<Machine>,
}

impl Solve for Solution {
    type Answer1 = i64;
    type Answer2 = i64;
    type Error = Error;

    fn new<R>(r: R) -> Result<Self, Error>
    where
        R: Read,
    {
        Ok(Self {
            machines: BufReader::new(r)
                .lines()
                .collect::<Result<Vec<_>, _>>()?
                .split(String::is_empty)
                .map(Machine::try_from)
                .collect::<Result<_, _>>()?,
        })
    }
    fn part1(&self) -> Self::Answer1 {
        self.machines
            .iter()
            .filter_map(|machine| machine.tokens(0))
            .sum()
    }
    fn part2(&self) -> Self::Answer2 {
        self.machines
            .iter()
            .filter_map(|machine| machine.tokens(10_000_000_000_000))
            .sum()
    }
}

fn main() -> Result<(), Error> {
    run::<Solution>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static [u8] {
        r"
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part1(), 480);
        Ok(())
    }
}
