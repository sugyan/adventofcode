use aoc2024::{Day, run_day};
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
    #[error("invalid input")]
    InvalidInput,
}

#[derive(Debug)]
struct XY {
    x: i64,
    y: i64,
}

impl FromStr for XY {
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
    a: XY,
    b: XY,
    prize: XY,
}

impl Machine {
    fn tokens(&self, offset: i64) -> Option<i64> {
        let det = |[[a, b], [c, d]]: [[i64; 2]; 2]| a * d - b * c;
        let d = det([[self.a.x, self.b.x], [self.a.y, self.b.y]]);
        if d == 0 {
            return None;
        }
        let d_a = det([
            [self.prize.x + offset, self.b.x],
            [self.prize.y + offset, self.b.y],
        ]);
        let d_b = det([
            [self.a.x, self.prize.x + offset],
            [self.a.y, self.prize.y + offset],
        ]);
        if d_a % d == 0 && d_b % d == 0 {
            let (a, b) = (d_a / d, d_b / d);
            Some(a * 3 + b)
        } else {
            None
        }
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

struct Input(Vec<Machine>);

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(String::from)
                .collect::<Vec<_>>()
                .split(String::is_empty)
                .map(Machine::try_from)
                .collect::<Result<_, _>>()?,
        ))
    }
}

struct Solution;

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = i64;
    type Answer2 = i64;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        input.0.iter().filter_map(|machine| machine.tokens(0)).sum()
    }
    fn part2(input: &Self::Input) -> Self::Answer2 {
        input
            .0
            .iter()
            .filter_map(|machine| machine.tokens(10_000_000_000_000))
            .sum()
    }
}

fn main() -> Result<(), aoc2024::Error<Error>> {
    run_day::<Solution>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> Result<Input, Error> {
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
"
        .trim_start()
        .parse()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::part1(&example_input()?), 480);
        Ok(())
    }
}
