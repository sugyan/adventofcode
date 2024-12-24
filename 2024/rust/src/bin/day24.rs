use aoc2024::{run, Solve};
use itertools::Itertools;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
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

#[derive(Debug)]
enum Gate {
    And(String, String),
    Or(String, String),
    Xor(String, String),
}

struct Solution {
    initial_values: HashMap<String, u8>,
    connections: HashMap<String, Gate>,
}

impl Solution {
    fn get_value(&self, name: &str) -> u8 {
        if self.initial_values.contains_key(name) {
            self.initial_values[name]
        } else {
            match &self.connections[name] {
                Gate::And(lhs, rhs) => self.get_value(lhs) & self.get_value(rhs),
                Gate::Or(lhs, rhs) => self.get_value(lhs) | self.get_value(rhs),
                Gate::Xor(lhs, rhs) => self.get_value(lhs) ^ self.get_value(rhs),
            }
        }
    }
}

impl Solve for Solution {
    type Answer1 = u64;
    type Answer2 = u64;
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
            .and_then(|(lines0, lines1)| {
                let (mut initial_values, mut connections) = (HashMap::new(), HashMap::new());
                for line in lines0 {
                    let (name, value) = line.split_once(": ").ok_or(Error::InvalidLine)?;
                    initial_values.insert(name.to_string(), value.parse()?);
                }
                for line in lines1 {
                    let (input, output) = line.split_once(" -> ").ok_or(Error::InvalidLine)?;
                    let (lhs, operator, rhs) = input
                        .split_ascii_whitespace()
                        .collect_tuple()
                        .ok_or(Error::InvalidLine)?;
                    let gate = match operator {
                        "AND" => Gate::And(lhs.to_string(), rhs.to_string()),
                        "OR" => Gate::Or(lhs.to_string(), rhs.to_string()),
                        "XOR" => Gate::Xor(lhs.to_string(), rhs.to_string()),
                        _ => return Err(Error::InvalidLine),
                    };
                    connections.insert(output.to_string(), gate);
                }
                Ok(Self {
                    initial_values,
                    connections,
                })
            })
    }
    fn part1(&self) -> Self::Answer1 {
        self.connections
            .keys()
            .filter(|s| s.starts_with('z'))
            .sorted()
            .enumerate()
            .map(|(i, key)| (1 << i) * if self.get_value(key) > 0 { 1 } else { 0 })
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

    fn example_input_1() -> &'static [u8] {
        r"
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
"[1..]
            .as_bytes()
    }

    fn example_input_2() -> &'static [u8] {
        r"
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input_1())?.part1(), 4);
        assert_eq!(Solution::new(example_input_2())?.part1(), 2024);
        Ok(())
    }
}
