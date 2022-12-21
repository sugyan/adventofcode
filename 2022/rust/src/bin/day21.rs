use aoc2022::Solve;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::num::ParseIntError;
use std::str::FromStr;

enum Job {
    Number(i64),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

impl FromStr for Job {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();
        match parts.get(1) {
            Some(&"+") => Ok(Self::Add(parts[0].into(), parts[2].into())),
            Some(&"-") => Ok(Self::Sub(parts[0].into(), parts[2].into())),
            Some(&"*") => Ok(Self::Mul(parts[0].into(), parts[2].into())),
            Some(&"/") => Ok(Self::Div(parts[0].into(), parts[2].into())),
            _ => Ok(Self::Number(s.parse()?)),
        }
    }
}

enum Expression {
    Human,
    Number(i64),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
}

struct Solution {
    monkeys: HashMap<String, Job>,
}

impl Solution {
    fn yell_number(&self, name: &str) -> i64 {
        match &self.monkeys[name] {
            Job::Number(n) => *n,
            Job::Add(lhs, rhs) => self.yell_number(lhs) + self.yell_number(rhs),
            Job::Sub(lhs, rhs) => self.yell_number(lhs) - self.yell_number(rhs),
            Job::Mul(lhs, rhs) => self.yell_number(lhs) * self.yell_number(rhs),
            Job::Div(lhs, rhs) => self.yell_number(lhs) / self.yell_number(rhs),
        }
    }
    fn eval(&self, name: &str) -> Expression {
        if name == "humn" {
            return Expression::Human;
        }
        match &self.monkeys[name] {
            Job::Number(n) => Expression::Number(*n),
            Job::Add(lhs, rhs) => match (self.eval(lhs), self.eval(rhs)) {
                (Expression::Number(nl), Expression::Number(nr)) => Expression::Number(nl + nr),
                (lhs, rhs) => Expression::Add(Box::new(lhs), Box::new(rhs)),
            },
            Job::Sub(lhs, rhs) => match (self.eval(lhs), self.eval(rhs)) {
                (Expression::Number(nl), Expression::Number(nr)) => Expression::Number(nl - nr),
                (lhs, rhs) => Expression::Sub(Box::new(lhs), Box::new(rhs)),
            },
            Job::Mul(lhs, rhs) => match (self.eval(lhs), self.eval(rhs)) {
                (Expression::Number(nl), Expression::Number(nr)) => Expression::Number(nl * nr),
                (lhs, rhs) => Expression::Mul(Box::new(lhs), Box::new(rhs)),
            },
            Job::Div(lhs, rhs) => match (self.eval(lhs), self.eval(rhs)) {
                (Expression::Number(nl), Expression::Number(nr)) => Expression::Number(nl / nr),
                (lhs, rhs) => Expression::Div(Box::new(lhs), Box::new(rhs)),
            },
        }
    }
    fn solve(exp: Expression, target: i64) -> i64 {
        match exp {
            Expression::Human => target,
            Expression::Add(lhs, rhs) => match (*lhs, *rhs) {
                (Expression::Number(n), exp) => Self::solve(exp, target - n),
                (exp, Expression::Number(n)) => Self::solve(exp, target - n),
                _ => unreachable!(),
            },
            Expression::Sub(lhs, rhs) => match (*lhs, *rhs) {
                (Expression::Number(n), exp) => Self::solve(exp, n - target),
                (exp, Expression::Number(n)) => Self::solve(exp, n + target),
                _ => unreachable!(),
            },
            Expression::Mul(lhs, rhs) => match (*lhs, *rhs) {
                (Expression::Number(n), exp) => Self::solve(exp, target / n),
                (exp, Expression::Number(n)) => Self::solve(exp, target / n),
                _ => unreachable!(),
            },
            Expression::Div(lhs, rhs) => match (*lhs, *rhs) {
                (Expression::Number(n), exp) => Self::solve(exp, n / target),
                (exp, Expression::Number(n)) => Self::solve(exp, n * target),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

impl Solve for Solution {
    type Answer1 = i64;
    type Answer2 = i64;

    fn new(r: impl Read) -> Self {
        Self {
            monkeys: BufReader::new(r)
                .lines()
                .filter_map(Result::ok)
                .filter_map(|line| {
                    line.split_once(": ")
                        .map(|(name, job)| (name.into(), job.parse().unwrap()))
                })
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.yell_number("root")
    }
    fn part2(&self) -> Self::Answer2 {
        let (lhs, rhs) = match &self.monkeys["root"] {
            Job::Add(lhs, rhs) => (lhs, rhs),
            _ => unreachable!(),
        };
        match (self.eval(lhs), self.eval(rhs)) {
            (Expression::Number(n), exp) | (exp, Expression::Number(n)) => Self::solve(exp, n),
            _ => unreachable!(),
        }
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
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(152, Solution::new(example_input()).part1());
    }

    #[test]
    fn part2() {
        assert_eq!(301, Solution::new(example_input()).part2());
    }
}
