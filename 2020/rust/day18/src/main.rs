use std::io::{BufRead, BufReader};
use std::str::Chars;

struct Solution {
    expressions: Vec<String>,
}

#[derive(Copy, Clone)]
enum Op {
    Add,
    Mul,
}

struct Term {
    op: Op,
    val: u64,
}

impl Term {
    fn new(op: Op, val: u64) -> Self {
        Self { op, val }
    }
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        Self {
            expressions: inputs.iter().map(String::to_string).collect(),
        }
    }
    fn part_1(&self) -> u64 {
        self.expressions
            .iter()
            .map(|expression| Solution::evaluate(expression, false))
            .sum()
    }
    fn part_2(&self) -> u64 {
        self.expressions
            .iter()
            .map(|expression| Solution::evaluate(expression, true))
            .sum()
    }
    fn evaluate(expression: &str, advanced: bool) -> u64 {
        let mut chars = expression.chars();
        Self::evaluate_recursive(&mut chars, advanced)
    }
    fn evaluate_recursive(chars: &mut Chars, advanced: bool) -> u64 {
        let mut v = Vec::new();
        let mut op = Op::Mul;
        while let Some(c) = chars.next() {
            match c {
                '0'..='9' => v.push(Term::new(op, u64::from(c as u8 - b'0'))),
                '+' => op = Op::Add,
                '*' => op = Op::Mul,
                '(' => v.push(Term::new(op, Self::evaluate_recursive(chars, advanced))),
                ')' => break,
                _ => {}
            }
        }
        if advanced {
            let mut ret = 1;
            while let Some(last) = v.pop() {
                match last.op {
                    Op::Add => {
                        if let Some(prev) = v.last_mut() {
                            prev.val += last.val;
                        }
                    }
                    Op::Mul => ret *= last.val,
                }
            }
            ret
        } else {
            v.iter().fold(1, |acc, x| match x.op {
                Op::Add => acc + x.val,
                Op::Mul => acc * x.val,
            })
        }
    }
}

fn main() {
    let solution = Solution::new(
        &BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(Result::ok)
            .collect::<Vec<_>>(),
    );
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(71, Solution::evaluate("1 + 2 * 3 + 4 * 5 + 6", false));
        assert_eq!(51, Solution::evaluate("1 + (2 * 3) + (4 * (5 + 6))", false));
        assert_eq!(26, Solution::evaluate("2 * 3 + (4 * 5)", false));
        assert_eq!(
            437,
            Solution::evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)", false)
        );
        assert_eq!(
            12240,
            Solution::evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", false)
        );
        assert_eq!(
            13632,
            Solution::evaluate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", false)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(231, Solution::evaluate("1 + 2 * 3 + 4 * 5 + 6", true));
        assert_eq!(51, Solution::evaluate("1 + (2 * 3) + (4 * (5 + 6))", true));
        assert_eq!(46, Solution::evaluate("2 * 3 + (4 * 5)", true));
        assert_eq!(
            1445,
            Solution::evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)", true)
        );
        assert_eq!(
            669_060,
            Solution::evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", true)
        );
        assert_eq!(
            23340,
            Solution::evaluate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", true)
        );
    }
}
