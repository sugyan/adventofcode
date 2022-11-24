use aoc2021::Solve;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};

enum Score {
    Incomplete(u64),
    Corrupted(u64),
}

struct Solution {
    scores: Vec<Score>,
}

impl Solve for Solution {
    type Answer1 = u64;
    type Answer2 = u64;

    fn new(r: impl Read) -> Self {
        let hm = HashMap::from([
            (')', ('(', 3)),
            (']', ('[', 57)),
            ('}', ('{', 1197)),
            ('>', ('<', 25137)),
        ]);
        Self {
            scores: BufReader::new(r)
                .lines()
                .filter_map(Result::ok)
                .map(|line| {
                    let mut stack = Vec::new();
                    for c in line.chars() {
                        if let Some((p, s)) = hm.get(&c) {
                            if stack.last() == Some(p) {
                                stack.pop();
                            } else {
                                return Score::Corrupted(*s);
                            }
                        } else {
                            stack.push(c)
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
    fn part1(&self) -> Self::Answer1 {
        self.scores
            .iter()
            .filter_map(|s| {
                if let Score::Corrupted(u) = s {
                    Some(*u)
                } else {
                    None
                }
            })
            .sum()
    }
    fn part2(&self) -> Self::Answer2 {
        let mut v = self
            .scores
            .iter()
            .filter_map(|s| {
                if let Score::Incomplete(u) = s {
                    Some(*u)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v[v.len() / 2]
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
            .as_bytes()
    }

    #[test]
    fn example1() {
        assert_eq!(26397, Solution::new(example_input()).part1());
    }

    #[test]
    fn example2() {
        assert_eq!(288957, Solution::new(example_input()).part2());
    }
}
