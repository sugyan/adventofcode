use std::io::{BufRead, BufReader};
use std::str::Chars;

struct Solution {
    inputs: Vec<String>,
}

#[derive(Copy, Clone)]
enum Op {
    Add,
    Mul,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        Self { inputs }
    }
    fn solve_1(&self) -> u64 {
        self.inputs
            .iter()
            .map(|s| Solution::evaluate(s, false))
            .sum()
    }
    fn solve_2(&self) -> u64 {
        self.inputs
            .iter()
            .map(|s| Solution::evaluate(s, true))
            .sum()
    }
    fn evaluate(expression: &str, advanced: bool) -> u64 {
        let mut chars = expression.chars();
        Solution::helper(&mut chars, advanced)
    }
    fn helper(chars: &mut Chars, advanced: bool) -> u64 {
        let mut v: Vec<(Op, u64)> = Vec::new();
        let mut op = Op::Mul;
        while let Some(c) = chars.next() {
            match c {
                '0'..='9' => v.push((op, (c as u8 - b'0') as u64)),
                '+' => op = Op::Add,
                '*' => op = Op::Mul,
                '(' => v.push((op, Solution::helper(chars, advanced))),
                ')' => break,
                _ => {}
            }
        }
        if advanced {
            let mut ret = 1;
            while let Some(last) = v.pop() {
                match last.0 {
                    Op::Add => {
                        if let Some(prev) = v.last_mut() {
                            prev.1 += last.1;
                        }
                    }
                    Op::Mul => ret *= last.1,
                }
            }
            ret
        } else {
            v.iter().fold(1, |acc, x| match x.0 {
                Op::Add => acc + x.1,
                Op::Mul => acc * x.1,
            })
        }
    }
}

fn main() {
    let solution = Solution::new(
        BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(|line| line.ok())
            .collect(),
    );
    println!("{}", solution.solve_1());
    println!("{}", solution.solve_2());
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
            669060,
            Solution::evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", true)
        );
        assert_eq!(
            23340,
            Solution::evaluate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", true)
        );
    }
}
