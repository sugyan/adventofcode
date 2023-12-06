use aoc2023::Solve;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    races: Vec<(u32, u32)>,
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        let lines = BufReader::new(r)
            .lines()
            .map_while(Result::ok)
            .collect::<Vec<_>>();
        let times = lines[0]
            .strip_prefix("Time:")
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap());
        let distances = lines[1]
            .strip_prefix("Distance:")
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap());
        Self {
            races: times.zip(distances).collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.races
            .iter()
            .map(|&(t, d)| (1..t).filter(|i| i * (t - i) > d).count())
            .product()
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
Time:      7  15   30
Distance:  9  40  200
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(Solution::new(example_input()).part1(), 288);
    }
}
