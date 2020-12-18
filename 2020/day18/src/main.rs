use std::io::{BufRead, BufReader};

struct Solution {
    inputs: Vec<String>,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        Self { inputs }
    }
    fn solve_1(&self) -> u64 {
        self.inputs.iter().map(|s| Solution::evaluate(s)).sum()
    }
    fn evaluate(expression: &str) -> u64 {
        let v: &[u8] = expression.as_bytes();
        let (mut op, mut n) = (b'+', 0);
        let mut ret = 0;
        let mut i = 0;
        while i < v.len() {
            match v[i] {
                b'0'..=b'9' => n = n * 10 + (v[i] - b'0') as u64,
                b'+' | b'*' => {
                    ret = if op == b'+' { ret + n } else { ret * n };
                    n = 0;
                    op = v[i];
                }
                b'(' => {
                    let (mut j, mut depth) = (i + 1, 1);
                    while depth > 0 {
                        depth += match v[j] {
                            b'(' => 1,
                            b')' => -1,
                            _ => 0,
                        };
                        j += 1;
                    }
                    n = Solution::evaluate(&expression[i + 1..j - 1]);
                    i = j;
                }
                _ => {}
            }
            i += 1;
        }
        if op == b'+' {
            ret + n
        } else {
            ret * n
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(71, Solution::evaluate("1 + 2 * 3 + 4 * 5 + 6"));
        assert_eq!(51, Solution::evaluate("1 + (2 * 3) + (4 * (5 + 6))"));
        assert_eq!(26, Solution::evaluate("2 * 3 + (4 * 5)"));
        assert_eq!(437, Solution::evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
        assert_eq!(
            12240,
            Solution::evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
        );
        assert_eq!(
            13632,
            Solution::evaluate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
        );
    }
}
