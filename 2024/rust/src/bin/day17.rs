use aoc2024::{Day, run_day};
use itertools::Itertools;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
    #[error("invalid input")]
    InvalidInput,
    #[error("invalid line")]
    InvalidLine,
}

#[derive(Debug, Default, Clone, Copy)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

impl Registers {
    fn execute(&mut self, instructions: &[u64]) -> Vec<u64> {
        let mut outputs = Vec::new();
        let mut i = 0;
        while i < instructions.len() - 1 {
            let (opcode, operand) = (instructions[i], instructions[i + 1]);
            match opcode {
                #[allow(clippy::assign_op_pattern)]
                0 => self.a = self.a >> self.combo_operand(operand),
                1 => self.b ^= operand,
                2 => self.b = self.combo_operand(operand) % 8,
                3 if self.a > 0 => {
                    i = operand as usize;
                    continue;
                }
                4 => self.b ^= self.c,
                5 => outputs.push(self.combo_operand(operand) % 8),
                6 => self.b = self.a >> self.combo_operand(operand),
                7 => self.c = self.a >> self.combo_operand(operand),
                _ => {}
            }
            i += 2;
        }
        outputs
    }
    fn combo_operand(&self, u: u64) -> u64 {
        match u {
            0..=3 => u,
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

struct Program(Vec<u64>);

impl FromStr for Program {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(": ")
            .ok_or(Error::InvalidLine)
            .and_then(|(_, s)| {
                Ok(Self(
                    s.split(',')
                        .map(str::parse)
                        .collect::<Result<Vec<_>, _>>()?,
                ))
            })
    }
}

struct Input {
    registers: Registers,
    program: Program,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .map(String::from)
            .collect_vec()
            .split(String::is_empty)
            .collect_tuple()
            .ok_or(Error::InvalidInput)
            .and_then(|(lines0, lines1)| {
                Ok(Self {
                    registers: lines0.try_into()?,
                    program: lines1.first().ok_or(Error::InvalidInput)?.parse()?,
                })
            })
    }
}

struct Solution;

impl Solution {
    fn dfs(input: &Input, curr: u64, i: usize) -> Option<u64> {
        for j in 0..8 {
            let value = curr + j * (1 << (i * 3));
            let mut registers = input.registers;
            registers.a = value;
            let outputs = registers.execute(&input.program.0);
            if outputs.len() == input.program.0.len() && outputs[i] == input.program.0[i] {
                if i == 0 {
                    return Some(value);
                }
                if let Some(ret) = Self::dfs(input, curr + j * (1 << (i * 3)), i - 1) {
                    return Some(ret);
                }
            }
        }
        None
    }
}

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = String;
    type Answer2 = u64;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        let mut registers = input.registers;
        registers.execute(&input.program.0).iter().join(",")
    }
    fn part2(input: &Self::Input) -> Self::Answer2 {
        Solution::dfs(input, 0, input.program.0.len() - 1).unwrap()
    }
}

fn main() -> Result<(), aoc2024::Error<Error>> {
    run_day::<Solution>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input_1() -> Result<Input, Error> {
        r"
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"
        .trim_start()
        .parse()
    }

    fn example_input_2() -> Result<Input, Error> {
        r"
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
"
        .trim_start()
        .parse()
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
        assert_eq!(Solution::part1(&example_input_1()?), "4,6,3,5,6,3,5,2,1,0");
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::part2(&example_input_2()?), 117440);
        Ok(())
    }
}
