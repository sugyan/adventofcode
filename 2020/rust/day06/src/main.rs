use std::io::{BufRead, BufReader};

struct Solution {
    groups: Vec<Vec<u32>>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        Self {
            groups: inputs
                .split(String::is_empty)
                .map(|lines| {
                    lines
                        .iter()
                        .map(|line| {
                            line.as_bytes()
                                .iter()
                                .map(|&b| 1 << (b - b'a') as usize)
                                .fold(0, |acc, x| acc | x)
                        })
                        .collect()
                })
                .collect(),
        }
    }
    fn part_1(&self) -> usize {
        self.groups
            .iter()
            .map(|group| group.iter().fold(0, |acc, &x| acc | x).count_ones() as usize)
            .sum()
    }
    fn part_2(&self) -> usize {
        self.groups
            .iter()
            .map(|group| {
                group
                    .iter()
                    .fold((1 << 26) - 1, |acc, &x| acc & x)
                    .count_ones() as usize
            })
            .sum()
    }
}

fn main() {
    let solution = Solution::new(
        &BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(Result::ok)
            .collect::<Vec<_>>(),
    );
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_inputs() -> Vec<String> {
        r"
abc

a
b
c

ab
ac

a
a
a
a

b"
        .split('\n')
        .skip(1)
        .map(str::to_string)
        .collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(11, Solution::new(&example_inputs()).part_1());
    }

    #[test]
    fn example_2() {
        assert_eq!(6, Solution::new(&example_inputs()).part_2());
    }
}
