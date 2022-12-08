use aoc2021::Solve;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    positions: Vec<i32>,
}

impl Solution {
    fn total_fuels(&self, constant: bool) -> i32 {
        let min = *self.positions.iter().min().unwrap();
        let max = *self.positions.iter().max().unwrap();
        (min..=max)
            .map(|i| {
                self.positions
                    .iter()
                    .map(|p| {
                        let d = (p - i).abs();
                        match constant {
                            true => d,
                            false => d * (d + 1) / 2,
                        }
                    })
                    .sum()
            })
            .min()
            .unwrap()
    }
}

impl Solve for Solution {
    type Answer1 = i32;
    type Answer2 = i32;

    fn new(r: impl Read) -> Self {
        Self {
            positions: BufReader::new(r)
                .lines()
                .find_map(Result::ok)
                .unwrap()
                .split(',')
                .filter_map(|x| x.parse().ok())
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.total_fuels(true)
    }
    fn part2(&self) -> Self::Answer2 {
        self.total_fuels(false)
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
16,1,2,0,4,2,7,1,2,14
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(37, Solution::new(example_input()).part1());
    }

    #[test]
    fn part2() {
        assert_eq!(168, Solution::new(example_input()).part2());
    }
}
