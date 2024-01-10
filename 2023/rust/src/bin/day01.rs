use aoc2023::{run, Solve};
use std::io::{BufRead, BufReader, Read};

#[cfg(not(feature = "opt"))]
mod solution {
    use super::*;

    pub struct Solution {
        input: Vec<String>,
    }

    impl Solution {
        fn calibration_value(line: &str, include_spelled: bool) -> u32 {
            let mut digits = (1..=9).map(|i| (i.to_string(), i)).collect::<Vec<_>>();
            if include_spelled {
                digits.extend([
                    ("one".into(), 1),
                    ("two".into(), 2),
                    ("three".into(), 3),
                    ("four".into(), 4),
                    ("five".into(), 5),
                    ("six".into(), 6),
                    ("seven".into(), 7),
                    ("eight".into(), 8),
                    ("nine".into(), 9),
                ]);
            }
            let mut v = Vec::new();
            for i in 0..line.len() {
                for (s, u) in &digits {
                    if line[i..].starts_with(s) {
                        v.push(*u);
                    }
                }
            }
            v[0] * 10 + v[v.len() - 1]
        }
    }

    impl Solve for Solution {
        type Answer1 = u32;
        type Answer2 = u32;

        fn new(r: impl Read) -> Self {
            Self {
                input: BufReader::new(r).lines().map_while(Result::ok).collect(),
            }
        }
        fn part1(&self) -> Self::Answer1 {
            self.input
                .iter()
                .map(|s| Self::calibration_value(s, false))
                .sum()
        }
        fn part2(&self) -> Self::Answer2 {
            self.input
                .iter()
                .map(|s| Self::calibration_value(s, true))
                .sum()
        }
    }
}

#[cfg(feature = "opt")]
mod solution {
    use super::*;
    use std::array;

    struct Trie {
        children: [Option<Box<Self>>; 128],
        digit: Option<u32>,
    }

    impl Trie {
        fn new(digits: &[(u32, String)]) -> Self {
            let mut trie = Self::default();
            for (digit, s) in digits {
                let mut node = &mut trie;
                for b in s.bytes() {
                    node = node.children[b as usize].get_or_insert_with(Box::default);
                }
                node.digit = Some(*digit);
            }
            trie
        }
        fn search(&self, s: &str) -> Option<u32> {
            let mut node = self;
            for b in s.bytes() {
                if let Some(child) = &node.children[b as usize] {
                    if child.digit.is_some() {
                        return child.digit;
                    }
                    node = child;
                } else {
                    return None;
                }
            }
            node.digit
        }
    }

    impl Default for Trie {
        fn default() -> Self {
            Self {
                children: array::from_fn(|_| None),
                digit: None,
            }
        }
    }

    pub struct Solution {
        input: Vec<String>,
        trie1: Trie,
        trie2: Trie,
    }

    impl Solution {
        const LETTERS: [&str; 9] = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        fn calibration_value(&self, line: &str, include_spelled: bool) -> u32 {
            let trie = if include_spelled {
                &self.trie2
            } else {
                &self.trie1
            };
            let (mut first, mut last) = (0, 0);
            for i in 0..line.len() {
                if let Some(d) = trie.search(&line[i..]) {
                    if first == 0 {
                        first = d;
                    }
                    last = d;
                }
            }
            first * 10 + last
        }
    }

    impl Solve for Solution {
        type Answer1 = u32;
        type Answer2 = u32;

        fn new(r: impl Read) -> Self {
            let digits = (1..=9).map(|i| (i, i.to_string())).collect::<Vec<_>>();
            let trie1 = Trie::new(&digits);
            let trie2 = Trie::new(
                &[
                    digits,
                    (1..).zip(Self::LETTERS.map(Into::into)).collect::<Vec<_>>(),
                ]
                .concat(),
            );
            Self {
                input: BufReader::new(r).lines().map_while(Result::ok).collect(),
                trie1,
                trie2,
            }
        }
        fn part1(&self) -> Self::Answer1 {
            self.input
                .iter()
                .map(|s| self.calibration_value(s, false))
                .sum()
        }
        fn part2(&self) -> Self::Answer2 {
            self.input
                .iter()
                .map(|s| self.calibration_value(s, true))
                .sum()
        }
    }
}

use solution::Solution;

fn main() {
    run(&Solution::new(std::io::stdin().lock()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            Solution::new(
                r"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"[1..]
                    .as_bytes()
            )
            .part1(),
            142
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            Solution::new(
                r"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"[1..]
                    .as_bytes()
            )
            .part2(),
            281,
        );
    }
}
