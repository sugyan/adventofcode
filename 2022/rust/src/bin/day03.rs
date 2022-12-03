use aoc2022::Solve;
use std::io::{BufRead, BufReader};

struct Solution {
    items: Vec<(u64, u64)>,
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl std::io::Read) -> Self {
        let b2i = |b| b - 38 - 58 * u8::from(b > 96);
        Self {
            items: BufReader::new(r)
                .lines()
                .filter_map(Result::ok)
                .map(|s| {
                    let (s1, s2) = s.split_at(s.len() / 2);
                    (
                        s1.bytes().fold(0, |acc, x| acc | 1 << b2i(x)),
                        s2.bytes().fold(0, |acc, x| acc | 1 << b2i(x)),
                    )
                })
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.items
            .iter()
            .map(|(i1, i2)| (i1 & i2).trailing_zeros())
            .sum()
    }
    fn part2(&self) -> Self::Answer2 {
        self.items
            .chunks(3)
            .map(|group| {
                group
                    .iter()
                    .fold(!0, |acc, x| acc & (x.0 | x.1))
                    .trailing_zeros()
            })
            .sum()
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

    #[test]
    fn example2() {
        assert_eq!(70, Solution::new(example_input()).part2());
    }
}
