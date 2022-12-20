use aoc2022::Solve;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    numbers: Vec<i64>,
}

impl Solution {
    fn mixed_indices(numbers: &[i64], times: usize) -> Vec<(usize, usize)> {
        let mut indices = (0..numbers.len())
            .map(|i| {
                (
                    (i + numbers.len() - 1) % numbers.len(),
                    (i + 1) % numbers.len(),
                )
            })
            .collect::<Vec<_>>();
        for _ in 0..times {
            for i in 0..numbers.len() {
                let mut j = i;
                match numbers[i].rem_euclid(numbers.len() as i64 - 1) {
                    0 => continue,
                    num => (0..num).for_each(|_| j = indices[j].1),
                }
                let (p, n) = indices[i];
                indices[p].1 = n;
                indices[n].0 = p;
                let k = indices[j].1;
                indices[j].1 = i;
                indices[k].0 = i;
                indices[i].0 = j;
                indices[i].1 = k;
            }
        }
        indices
    }
    fn grove_coordinates(numbers: &[i64], indices: &[(usize, usize)]) -> i64 {
        let mut i = 0;
        while numbers[i] != 0 {
            i = indices[i].1;
        }
        (0..3)
            .map(|_| {
                (0..1000).for_each(|_| i = indices[i].1);
                numbers[i]
            })
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
        Self::grove_coordinates(&self.numbers, &Self::mixed_indices(&self.numbers, 1))
    }
    fn part2(&self) -> Self::Answer2 {
        let numbers = self
            .numbers
            .iter()
            .map(|n| n * 811589153)
            .collect::<Vec<_>>();
        Self::grove_coordinates(&numbers, &Self::mixed_indices(&numbers, 10))
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
