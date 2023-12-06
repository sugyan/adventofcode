use aoc2023::Solve;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    times: Vec<String>,
    distances: Vec<String>,
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;

    fn new(r: impl Read) -> Self {
        let lines = BufReader::new(r)
            .lines()
            .map_while(Result::ok)
            .collect::<Vec<_>>();
        let times = lines[0]
            .strip_prefix("Time:")
            .unwrap()
            .split_whitespace()
            .map(String::from)
            .collect();
        let distances = lines[1]
            .strip_prefix("Distance:")
            .unwrap()
            .split_whitespace()
            .map(String::from)
            .collect();
        Self { times, distances }
    }
    fn part1(&self) -> Self::Answer1 {
        self.times
            .iter()
            .map(|s| s.parse().unwrap())
            .zip(self.distances.iter().map(|s| s.parse().unwrap()))
            .map(|(t, d)| (1..t).filter(|i| i * (t - i) > d).count())
            .product()
    }
    fn part2(&self) -> Self::Answer2 {
        let t = self.times.join("").parse::<u64>().unwrap();
        let d = self.distances.join("").parse::<u64>().unwrap();
        (1..t).filter(|i| i * (t - i) > d).count()
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

    #[test]
    fn part2() {
        assert_eq!(Solution::new(example_input()).part2(), 71503);
    }
}
