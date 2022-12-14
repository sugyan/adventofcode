use aoc2022::Solve;
use std::io::{BufRead, BufReader, Read};

#[derive(Clone)]
enum Operation {
    Add(u64),
    Mul(u64),
    Square,
}

#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: (u64, [usize; 2]),
}

impl From<&[String]> for Monkey {
    fn from(lines: &[String]) -> Self {
        Self {
            items: lines[1][18..]
                .split(", ")
                .filter_map(|s| s.parse().ok())
                .collect(),
            operation: match (&lines[2][23..24], &lines[2][25..]) {
                ("*", "old") => Operation::Square,
                ("*", s) => Operation::Mul(s.parse().unwrap()),
                ("+", s) => Operation::Add(s.parse().unwrap()),
                _ => unreachable!(),
            },
            test: (
                lines[3][21..].parse().unwrap(),
                [
                    lines[5][30..].parse().unwrap(),
                    lines[4][29..].parse().unwrap(),
                ],
            ),
        }
    }
}

struct Solution {
    monkeys: Vec<Monkey>,
}

impl Solution {
    fn monkey_business(&self, round: usize, divide3: bool) -> u64 {
        let mut monkeys = self.monkeys.clone();
        let mut inspected = vec![0; monkeys.len()];
        let div = monkeys.iter().map(|m| m.test.0).product::<u64>();
        for _ in 0..round {
            for i in 0..monkeys.len() {
                while let Some(item) = monkeys[i].items.pop() {
                    let level = match monkeys[i].operation {
                        Operation::Add(n) => item + n,
                        Operation::Mul(n) => item * n,
                        Operation::Square => item * item,
                    } / if divide3 { 3 } else { 1 }
                        % div;
                    let to = monkeys[i].test.1[usize::from(level % monkeys[i].test.0 == 0)];
                    monkeys[to].items.push(level);
                    inspected[i] += 1;
                }
            }
        }
        inspected.sort_unstable();
        inspected.iter().rev().take(2).product()
    }
}

impl Solve for Solution {
    type Answer1 = u64;
    type Answer2 = u64;

    fn new(r: impl Read) -> Self {
        Self {
            monkeys: BufReader::new(r)
                .lines()
                .filter_map(Result::ok)
                .collect::<Vec<_>>()
                .split(String::is_empty)
                .map(Monkey::from)
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.monkey_business(20, true)
    }
    fn part2(&self) -> Self::Answer2 {
        self.monkey_business(10000, false)
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
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(10605, Solution::new(example_input()).part1());
    }

    #[test]
    fn part2() {
        assert_eq!(2_713_310_158, Solution::new(example_input()).part2());
    }
}
