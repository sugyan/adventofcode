use aoc2022::Solve;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    numbers: Vec<i64>,
}

impl Solution {
    fn grove_coordinates(numbers: &[i64], mix_times: usize) -> i64 {
        let mut indices = (0..numbers.len()).collect::<Vec<_>>();
        for _ in 0..mix_times {
            for (i, number) in numbers.iter().enumerate() {
                if let Some(j) = indices.iter().position(|&j| j == i) {
                    indices.remove(j);
                    let moves = number.rem_euclid(indices.len().try_into().unwrap()) as usize;
                    indices.insert((j + moves) % indices.len(), i);
                }
            }
        }
        let pos0 = numbers.iter().position(|&n| n == 0).unwrap();
        let i = indices.iter().position(|&i| i == pos0).unwrap();
        [1000, 2000, 3000]
            .iter()
            .map(|j| numbers[indices[(i + j) % numbers.len()]])
            .sum()
    }
}

impl Solve for Solution {
    type Answer1 = i64;
    type Answer2 = i64;

    fn new(r: impl Read) -> Self {
        Self {
            numbers: BufReader::new(r)
                .lines()
                .filter_map(Result::ok)
                .filter_map(|s| s.parse().ok())
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        Self::grove_coordinates(&self.numbers, 1)
    }
    fn part2(&self) -> Self::Answer2 {
        Self::grove_coordinates(
            &self
                .numbers
                .iter()
                .map(|n| n * 811_589_153)
                .collect::<Vec<_>>(),
            10,
        )
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
1
2
-3
3
-2
0
4
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(3, Solution::new(example_input()).part1());
    }

    #[test]
    fn part2() {
        assert_eq!(1_623_178_306, Solution::new(example_input()).part2());
    }
}
