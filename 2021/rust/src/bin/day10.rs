use std::io::{BufRead, BufReader};

enum Score {
    Incomplete(u64),
    Corrupted(u64),
}

struct Solution {
    scores: Vec<Score>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        Self {
            scores: inputs
                .iter()
                .map(|line| {
                    let mut stack = Vec::new();
                    for c in line.chars() {
                        match c {
                            ')' => {
                                if stack.last() == Some(&'(') {
                                    stack.pop();
                                } else {
                                    return Score::Corrupted(3);
                                }
                            }
                            ']' => {
                                if stack.last() == Some(&'[') {
                                    stack.pop();
                                } else {
                                    return Score::Corrupted(57);
                                }
                            }
                            '}' => {
                                if stack.last() == Some(&'{') {
                                    stack.pop();
                                } else {
                                    return Score::Corrupted(1197);
                                }
                            }
                            '>' => {
                                if stack.last() == Some(&'<') {
                                    stack.pop();
                                } else {
                                    return Score::Corrupted(25137);
                                }
                            }
                            c => stack.push(c),
                        }
                    }
                    Score::Incomplete(stack.iter().rev().fold(0, |acc, &c| {
                        acc * 5
                            + match c {
                                '(' => 1,
                                '[' => 2,
                                '{' => 3,
                                '<' => 4,
                                _ => unreachable!(),
                            }
                    }))
                })
                .collect(),
        }
    }
    fn part_1(&self) -> u64 {
        self.scores
            .iter()
            .filter_map(|s| match s {
                Score::Corrupted(u) => Some(*u),
                _ => None,
            })
            .sum()
    }
    fn part_2(&self) -> u64 {
        let mut v = self
            .scores
            .iter()
            .filter_map(|s| match s {
                Score::Incomplete(u) => Some(*u),
                _ => None,
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v[v.len() / 2]
    }
}

fn main() {
    let solution = Solution::new(
        &BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(Result::ok)
            .collect::<Vec<_>>(),
    );
    println!("{}", solution.part_1());
    println!("{}", solution.part_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_inputs() -> Vec<String> {
        r"
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"[1..]
            .split('\n')
            .map(String::from)
            .collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(26397, Solution::new(&example_inputs()).part_1());
    }

    #[test]
    fn example_2() {
        assert_eq!(288957, Solution::new(&example_inputs()).part_2());
    }
}
