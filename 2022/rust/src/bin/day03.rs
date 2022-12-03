use aoc2022::Solve;
use std::collections::HashSet;
use std::io::{BufRead, BufReader};

struct Solution {
    items: Vec<String>,
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl std::io::Read) -> Self {
        Self {
            items: BufReader::new(r).lines().filter_map(Result::ok).collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let mut answer = 0;
        for item in &self.items {
            let v = item.chars().collect::<Vec<_>>();
            let (v1, v2) = v.split_at(v.len() / 2);
            let hs1 = v1.iter().collect::<HashSet<_>>();
            let hs2 = v2.iter().collect::<HashSet<_>>();
            if let Some(&c) = hs1.intersection(&hs2).next() {
                answer += if c.is_lowercase() {
                    *c as u32 - 'a' as u32 + 1
                } else {
                    *c as u32 - 'A' as u32 + 27
                };
            }
        }
        answer
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
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"[1..]
            .as_bytes()
    }

    #[test]
    fn example1() {
        assert_eq!(157, Solution::new(example_input()).part1());
    }
}
