use aoc2021::Solve;
use std::cmp::Ordering;
use std::io::{BufRead, BufReader, Read};

enum Target {
    MostCommon,
    LeastCommon,
}

struct Solution {
    reports: Vec<u32>,
    size: usize,
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        let inputs = BufReader::new(r)
            .lines()
            .filter_map(Result::ok)
            .collect::<Vec<_>>();
        Self {
            size: inputs[0].len(),
            reports: inputs
                .iter()
                .map(|s| s.chars().fold(0, |acc, c| (acc << 1) + u32::from(c == '1')))
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let mut counts = vec![(0, 0); self.size];
        for report in &self.reports {
            for (i, c) in counts.iter_mut().rev().enumerate() {
                if report & (1 << i) == 0 {
                    c.0 += 1;
                } else {
                    c.1 += 1;
                }
            }
        }
        let (gamma, epsilon) = counts.iter().fold((0, 0), |(gamma, epsilon), c| {
            let d = match c.0.cmp(&c.1) {
                Ordering::Less => (1, 0),
                Ordering::Greater => (0, 1),
                Ordering::Equal => unreachable!(),
            };
            (gamma * 2 + d.0, epsilon * 2 + d.1)
        });
        gamma * epsilon
    }
    fn part2(&self) -> Self::Answer2 {
        let find_rating = |target: Target| {
            let mut candidates = self.reports.clone();
            for i in (0..self.size).rev() {
                let mask = 1 << i;
                let zeros = candidates.iter().filter(|&c| c & mask == 0).count();
                let t = match target {
                    Target::MostCommon => zeros * 2 > candidates.len(),
                    Target::LeastCommon => zeros * 2 <= candidates.len(),
                };
                candidates.retain(|&c| (c & mask == 0) == t);
                if candidates.len() == 1 {
                    break;
                }
            }
            candidates[0]
        };
        let (oxygen_generator, co2_scrubber) = (
            find_rating(Target::MostCommon),
            find_rating(Target::LeastCommon),
        );
        oxygen_generator * co2_scrubber
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
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"[1..]
            .as_bytes()
    }

    #[test]
    fn example1() {
        assert_eq!(198, Solution::new(example_input()).part1());
    }

    #[test]
    fn example2() {
        assert_eq!(230, Solution::new(example_input()).part2());
    }
}
