use aoc2021::Solve;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Variable {
    W,
    X,
    Y,
    Z,
}

impl From<&str> for Variable {
    fn from(s: &str) -> Self {
        match s {
            "w" => Self::W,
            "x" => Self::X,
            "y" => Self::Y,
            "z" => Self::Z,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VariableOrNumber {
    Variable(Variable),
    Number(i64),
}

impl From<&str> for VariableOrNumber {
    fn from(s: &str) -> Self {
        if let Ok(v) = s.parse() {
            Self::Number(v)
        } else {
            Self::Variable(s.into())
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Inp(Variable),
    Add(Variable, VariableOrNumber),
    Mul(Variable, VariableOrNumber),
    Div(Variable, VariableOrNumber),
    Mod(Variable, VariableOrNumber),
    Eql(Variable, VariableOrNumber),
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Default)]
struct ALU {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

impl ALU {
    fn variable_mut(&mut self, v: &Variable) -> &mut i64 {
        match v {
            Variable::W => &mut self.w,
            Variable::X => &mut self.x,
            Variable::Y => &mut self.y,
            Variable::Z => &mut self.z,
        }
    }
    fn get_value(&self, von: &VariableOrNumber) -> i64 {
        match von {
            VariableOrNumber::Variable(v) => match v {
                Variable::W => self.w,
                Variable::X => self.x,
                Variable::Y => self.y,
                Variable::Z => self.z,
            },
            VariableOrNumber::Number(n) => *n,
        }
    }
    fn apply(&mut self, inst: &Instruction, input: i64) {
        match inst {
            Instruction::Inp(v) => *self.variable_mut(v) = input,
            Instruction::Add(v1, v2) => *self.variable_mut(v1) += self.get_value(v2),
            Instruction::Mul(v1, v2) => *self.variable_mut(v1) *= self.get_value(v2),
            Instruction::Div(v1, v2) => *self.variable_mut(v1) /= self.get_value(v2),
            Instruction::Mod(v1, v2) => *self.variable_mut(v1) %= self.get_value(v2),
            Instruction::Eql(v1, v2) => {
                *self.variable_mut(v1) = i64::from(
                    self.get_value(&VariableOrNumber::Variable(*v1)) == self.get_value(v2),
                )
            }
        }
    }
}

struct Solution {
    instructions: Vec<Instruction>,
}

impl Solution {
    fn is_valid_input(&self, inputs: &[i64]) -> bool {
        let mut alu = ALU::default();
        let mut index = 0;
        let mut input = inputs[index];
        for inst in &self.instructions {
            alu.apply(inst, input);
            if let Instruction::Inp(_) = inst {
                index += 1;
                if index < inputs.len() {
                    input = inputs[index]
                }
            }
        }
        alu.z == 0 && index == inputs.len()
    }
    fn find_model_number(&self, rev: bool) -> i64 {
        let conditions = (0..14)
            .filter_map(|i| {
                if let (
                    Instruction::Add(_, VariableOrNumber::Number(n1)),
                    Instruction::Add(_, VariableOrNumber::Number(n2)),
                ) = (
                    self.instructions[i * 18 + 5],
                    self.instructions[i * 18 + 15],
                ) {
                    Some((n1, n2))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let mut inputs = Vec::new();
        let candidates = if rev {
            (1..=9).rev().collect::<Vec<_>>()
        } else {
            (1..=9).collect::<Vec<_>>()
        };
        Self::backtrack(&mut inputs, &mut Vec::new(), &conditions, &candidates);
        assert!(self.is_valid_input(&inputs));
        inputs.iter().fold(0, |acc, x| acc * 10 + x)
    }
    fn backtrack(
        inputs: &mut Vec<i64>,
        stack: &mut Vec<usize>,
        conditions: &[(i64, i64)],
        candidates: &[i64],
    ) -> bool {
        if inputs.len() == 14 {
            return true;
        }
        let (n, _) = conditions[inputs.len()];
        if n < 10 {
            if let Some(&index) = stack.last() {
                let i = inputs[index] + conditions[index].1 + n;
                if candidates.contains(&i) {
                    if let Some(last) = stack.pop() {
                        inputs.push(i);
                        if Self::backtrack(inputs, stack, conditions, candidates) {
                            return true;
                        }
                        inputs.pop();
                        stack.push(last);
                    }
                }
            }
        } else {
            for &i in candidates {
                stack.push(inputs.len());
                inputs.push(i);
                if Self::backtrack(inputs, stack, conditions, candidates) {
                    return true;
                }
                inputs.pop();
                stack.pop();
            }
        }
        false
    }
}

impl Solve for Solution {
    type Answer1 = i64;
    type Answer2 = i64;

    fn new(r: impl std::io::Read) -> Self {
        Self {
            instructions: BufReader::new(r)
                .lines()
                .filter_map(Result::ok)
                .map(|line| {
                    let parts = line.split(' ').collect::<Vec<_>>();
                    match parts[0] {
                        "inp" => Instruction::Inp(parts[1].into()),
                        "add" => Instruction::Add(parts[1].into(), parts[2].into()),
                        "mul" => Instruction::Mul(parts[1].into(), parts[2].into()),
                        "div" => Instruction::Div(parts[1].into(), parts[2].into()),
                        "mod" => Instruction::Mod(parts[1].into(), parts[2].into()),
                        "eql" => Instruction::Eql(parts[1].into(), parts[2].into()),
                        _ => unreachable!(),
                    }
                })
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.find_model_number(true)
    }
    fn part2(&self) -> Self::Answer2 {
        self.find_model_number(false)
    }
}

fn main() {
    let solution = Solution::new(std::io::stdin().lock());
    println!("Part 1: {}", solution.part1());
    println!("Part 2: {}", solution.part2());
}
