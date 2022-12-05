use aoc2022::Solve;
use itertools::Itertools;
use std::io::{BufRead, BufReader};

struct Solution {
    stacks: Vec<Vec<char>>,
    procedure: Vec<(usize, usize, usize)>,
}

impl Solution {
    fn move_crates(&self, stacks: &mut [Vec<char>], multiple_at_once: bool) {
        for &(count, from, to) in &self.procedure {
            let mut v = Vec::new();
            for _ in 0..count {
                if let Some(last) = stacks[from - 1].pop() {
                    v.push(last);
                }
            }
            if !multiple_at_once {
                v.reverse();
            }
            while let Some(last) = v.pop() {
                stacks[to - 1].push(last);
            }
        }
    }
}

impl Solve for Solution {
    type Answer1 = String;
    type Answer2 = String;

    fn new(r: impl std::io::Read) -> Self {
        let lines = BufReader::new(r)
            .lines()
            .filter_map(Result::ok)
            .collect::<Vec<_>>();
        let parts = lines.split(String::is_empty).collect::<Vec<_>>();
        let mut stacks = vec![Vec::new(); (parts[0][parts[0].len() - 1].len() + 1) / 4];
        for line in parts[0].iter().rev().skip(1) {
            for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
                if c != ' ' {
                    stacks[i].push(c);
                }
            }
        }
        Self {
            stacks,
            procedure: parts[1]
                .iter()
                .filter_map(|s| {
                    s.split_ascii_whitespace()
                        .skip(1)
                        .step_by(2)
                        .filter_map(|s| s.parse().ok())
                        .collect_tuple()
                })
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let mut stacks = self.stacks.clone();
        self.move_crates(&mut stacks, false);
        stacks.iter().filter_map(|s| s.last()).collect()
    }
    fn part2(&self) -> Self::Answer2 {
        let mut stacks = self.stacks.clone();
        self.move_crates(&mut stacks, true);
        stacks.iter().filter_map(|s| s.last()).collect()
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
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"[1..]
            .as_bytes()
    }

    #[test]
    fn example1() {
        assert_eq!("CMZ", Solution::new(example_input()).part1());
    }

    #[test]
    fn example2() {
        assert_eq!("MCD", Solution::new(example_input()).part2());
    }
}
