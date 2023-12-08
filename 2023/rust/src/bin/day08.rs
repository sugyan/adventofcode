use aoc2023::Solve;
use itertools::Itertools;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    instructions: String,
    network: HashMap<String, (String, String)>,
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        let lines = BufReader::new(r)
            .lines()
            .map_while(Result::ok)
            .collect::<Vec<_>>();
        Self {
            instructions: lines[0].clone(),
            network: lines[2..]
                .iter()
                .map(|line| {
                    line.split_once(" = ")
                        .map(|(label, nodes)| {
                            (
                                label.into(),
                                nodes
                                    .trim_matches(|c| c == '(' || c == ')')
                                    .split(", ")
                                    .map(String::from)
                                    .collect_tuple()
                                    .expect("invalid nodes"),
                            )
                        })
                        .expect("invalid line")
                })
                .collect(),
        }
    }

    fn part1(&self) -> Self::Answer1 {
        let (mut i, mut current) = (0, "AAA");
        let mut instructions = self.instructions.chars().cycle();
        while current != "ZZZ" {
            i += 1;
            let (l, r) = &self.network[current];
            current = match instructions.next() {
                Some('L') => l,
                Some('R') => r,
                _ => unreachable!(),
            }
        }
        i
    }
    fn part2(&self) -> Self::Answer2 {
        todo!()
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
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(Solution::new(example_input()).part1(), 6);
    }
}
