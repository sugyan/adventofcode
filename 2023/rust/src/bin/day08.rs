use aoc2023::Solve;
use itertools::Itertools;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

struct Solution {
    instructions: String,
    network: HashMap<String, (String, String)>,
}

impl Solution {
    fn num_steps(&self, ghosts: bool) -> u64 {
        self.network
            .keys()
            .filter(|k| {
                if ghosts {
                    k.ends_with('A')
                } else {
                    *k == "AAA"
                }
            })
            .map(String::as_str)
            .map(|s| self.find_cycle(s))
            .fold(1, lcm)
    }
    fn find_cycle(&self, start: &str) -> u64 {
        let (mut current, mut hm) = (start, HashMap::new());
        let mut instructions = self.instructions.chars().enumerate().cycle();
        for i in 1_u64.. {
            let (index, instruction) = instructions.next().expect("no instruction");
            let (l, r) = &self.network[current];
            current = match instruction {
                'L' => l,
                'R' => r,
                _ => unreachable!(),
            };
            if current == "ZZZ" {
                return i;
            }
            if let Some(p) = hm.get(&(index, current)) {
                return i - p;
            }
            hm.insert((index, current), i);
        }
        unreachable!()
    }
}

impl Solve for Solution {
    type Answer1 = u64;
    type Answer2 = u64;

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
        self.num_steps(false)
    }
    fn part2(&self) -> Self::Answer2 {
        self.num_steps(true)
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

    #[test]
    fn part1() {
        assert_eq!(
            Solution::new(
                r"
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"[1..]
                    .as_bytes()
            )
            .part1(),
            6
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            Solution::new(
                r"
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"[1..]
                    .as_bytes()
            )
            .part2(),
            6
        );
    }
}
