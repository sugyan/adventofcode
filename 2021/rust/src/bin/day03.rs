use std::cmp::Ordering;
use std::io::{BufRead, BufReader};

enum Target {
    MostCommon,
    LeastCommon,
}

struct Solution {
    reports: Vec<u32>,
    size: usize,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        Self {
            size: inputs[0].len(),
            reports: inputs
                .iter()
                .map(|s| {
                    s.chars()
                        .fold(0, |acc, c| (acc << 1) + if c == '1' { 1 } else { 0 })
                })
                .collect(),
        }
    }
    fn part_1(&self) -> u32 {
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
    fn part_2(&self) -> u32 {
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
    let solution = Solution::new(
        &BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(Result::ok)
            .collect::<Vec<_>>(),
    );
    println!("{}", solution.part_1());
    println!("{}", solution.part_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_inputs() -> Vec<String> {
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
01010"[1..]
            .split('\n')
            .map(String::from)
            .collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(198, Solution::new(&example_inputs()).part_1());
    }

    #[test]
    fn example_2() {
        assert_eq!(230, Solution::new(&example_inputs()).part_2());
    }
}
