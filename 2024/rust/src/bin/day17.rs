use aoc2024::{run, Solve};
use itertools::Itertools;
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
    #[error("invalid line")]
    InvalidLine,
}

#[derive(Debug, Default, Clone, Copy)]
struct Registers {
    a: u32,
    b: u32,
    c: u32,
}

impl Registers {
    fn execute(&mut self, instructions: &[u8]) -> Vec<u32> {
        let mut outputs = Vec::new();
        let mut i = 0;
        while i < instructions.len() - 1 {
            let opcode = &instructions[i];
            let operand = &instructions[i + 1];
            match opcode {
                #[allow(clippy::assign_op_pattern)]
                0 => self.a = self.a >> self.combo_operand(*operand),
                1 => self.b ^= u32::from(*operand),
                2 => self.b = self.combo_operand(*operand) % 8,
                3 if self.a > 0 => {
                    i = usize::from(*operand);
                    continue;
                }
                4 => self.b ^= self.c,
                5 => outputs.push(self.combo_operand(*operand) % 8),
                6 => self.b = self.a >> self.combo_operand(*operand),
                7 => self.c = self.a >> self.combo_operand(*operand),
                _ => {}
            }
            i += 2;
        }
        outputs
    }
    fn combo_operand(&self, u: u8) -> u32 {
        match u {
            0..=3 => u.into(),
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<&[String]> for Registers {
    type Error = Error;

    fn try_from(lines: &[String]) -> Result<Self, Self::Error> {
        let parse = |s: &String| {
            s.split_once(": ")
                .ok_or(Error::InvalidLine)
                .and_then(|(_, n)| Ok(n.parse()?))
        };
        Ok(Self {
            #[allow(clippy::get_first)]
            a: parse(lines.get(0).ok_or(Error::InvalidInput)?)?,
            b: parse(lines.get(1).ok_or(Error::InvalidInput)?)?,
            c: parse(lines.get(2).ok_or(Error::InvalidInput)?)?,
        })
    }
}

struct Program(Vec<u8>);

impl FromStr for Program {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(": ")
            .ok_or(Error::InvalidLine)
            .and_then(|(_, s)| {
                Ok(Self(
                    s.split(',')
                        .map(u8::from_str)
                        .collect::<Result<Vec<_>, _>>()?,
                ))
            })
    }
}

struct Solution {
    registers: Registers,
    program: Program,
}

impl Solve for Solution {
    type Answer1 = String;
    type Answer2 = String;
    type Error = Error;

    fn new<R>(r: R) -> Result<Self, Error>
    where
        R: Read,
    {
        BufReader::new(r)
            .lines()
            .collect::<Result<Vec<_>, _>>()?
            .split(String::is_empty)
            .collect_tuple()
            .ok_or(Error::InvalidInput)
            .and_then(|(registers, program)| {
                Ok(Self {
                    registers: registers.try_into()?,
                    program: program.first().ok_or(Error::InvalidInput)?.parse()?,
                })
            })
    }
    fn part1(&self) -> Self::Answer1 {
        let mut registers = self.registers;
        registers.execute(&self.program.0).iter().join(",")
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
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"[1..]
            .as_bytes()
    }

    #[test]
    fn operations() {
        {
            let mut registers = Registers {
                c: 9,
                ..Default::default()
            };
            registers.execute(&[2, 6]);
            assert_eq!(registers.b, 1);
        }
        {
            let mut registers = Registers {
                a: 10,
                ..Default::default()
            };
            assert_eq!(registers.execute(&[5, 0, 5, 1, 5, 4]), vec![0, 1, 2]);
        }
        {
            let mut registers = Registers {
                a: 2024,
                ..Default::default()
            };
            assert_eq!(
                registers.execute(&[0, 1, 5, 4, 3, 0]),
                vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]
            );
            assert_eq!(registers.a, 0);
        }
        {
            let mut registers = Registers {
                b: 29,
                ..Default::default()
            };
            registers.execute(&[1, 7]);
            assert_eq!(registers.b, 26);
        }
        {
            let mut registers = Registers {
                b: 2024,
                c: 43690,
                ..Default::default()
            };
            registers.execute(&[4, 0]);
            assert_eq!(registers.b, 44354);
        }
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(
            Solution::new(example_input())?.part1(),
            "4,6,3,5,6,3,5,2,1,0"
        );
        Ok(())
    }
}
