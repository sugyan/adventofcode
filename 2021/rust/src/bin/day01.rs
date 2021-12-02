use std::io::{BufRead, BufReader};

struct Solution {
    reports: Vec<u32>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        Self {
            reports: inputs.iter().filter_map(|s| s.parse().ok()).collect(),
        }
    }
    fn part_1(&self) -> usize {
        self.count_increases(1)
    }
    fn part_2(&self) -> usize {
        self.count_increases(3)
    }
    fn count_increases(&self, size: usize) -> usize {
        (size..self.reports.len())
            .filter(|&i| self.reports[i] > self.reports[i - size])
            .count()
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
199
200
208
210
200
207
240
269
260
263"[1..]
            .split('\n')
            .map(String::from)
            .collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(7, Solution::new(&example_inputs()).part_1());
    }

    #[test]
    fn example_2() {
        assert_eq!(5, Solution::new(&example_inputs()).part_2());
    }
}
