use aoc2022::Solve;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
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

struct Solution {
    monkeys: HashMap<String, Job>,
}

impl Solution {
    fn yell_number(&self, name: &str) -> i64 {
        match &self.monkeys[name] {
            Job::Number(n) => *n,
            Job::Add(a, b) => self.yell_number(a) + self.yell_number(b),
            Job::Sub(a, b) => self.yell_number(a) - self.yell_number(b),
            Job::Mul(a, b) => self.yell_number(a) * self.yell_number(b),
            Job::Div(a, b) => self.yell_number(a) / self.yell_number(b),
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
        todo!()
    }
}

fn main() {
    let solution = Solution::new(std::io::stdin().lock());
    println!("Part 1: {}", solution.part1());
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
}
