use aoc2024::{run, Solve};
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

fn parse_line(line: &str) -> Result<(usize, usize), Error> {
    line.split_once(": ")
        .and_then(|(_, s)| s.split_once(", "))
        .ok_or(Error::InvalidInput)
        .and_then(|(x, y)| Ok((x[2..].parse()?, y[2..].parse()?)))
}

#[derive(Debug)]
struct Button {
    x: usize,
    y: usize,
}

impl FromStr for Button {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_line(s).map(|(x, y)| Self { x, y })
    }
}

#[derive(Debug)]
struct Prize {
    x: usize,
    y: usize,
}

impl FromStr for Prize {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_line(s).map(|(x, y)| Self { x, y })
    }
}

#[derive(Debug)]
struct Machine {
    a: Button,
    b: Button,
    prize: Prize,
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
    type Answer1 = usize;
    type Answer2 = usize;
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
            .filter_map(|machine| {
                for a in 0..100 {
                    for b in 0..100 {
                        if machine.a.x * a + machine.b.x * b == machine.prize.x
                            && machine.a.y * a + machine.b.y * b == machine.prize.y
                        {
                            return Some(a * 3 + b);
                        }
                    }
                }
                None
            })
            .sum()
    }
    fn part2(&self) -> Self::Answer2 {
        todo!()
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
