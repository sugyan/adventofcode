use std::collections::HashSet;
use std::io::{BufRead, BufReader};

struct Solution {
    reports: Vec<i32>,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        Self {
            reports: inputs.iter().filter_map(|s| s.parse().ok()).collect(),
        }
    }
    fn part_1(&self) -> i32 {
        let hs = self.reports.iter().collect::<HashSet<_>>();
        for &i in self.reports.iter() {
            if hs.contains(&(2020 - i)) {
                return (2020 - i) * i;
            }
        }
        unreachable!()
    }
    fn part_2(&self) -> i32 {
        let hs: HashSet<&i32> = self.reports.iter().collect();
        for i in 0..self.reports.len() - 1 {
            for j in i + 1..self.reports.len() {
                if hs.contains(&(2020 - self.reports[i] - self.reports[j])) {
                    return (2020 - self.reports[i] - self.reports[j])
                        * self.reports[i]
                        * self.reports[j];
                }
            }
        }
        unreachable!()
    }
}

fn main() {
    let solution = Solution::new(
        BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(|line| line.ok())
            .collect(),
    );
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_inputs() -> Vec<String> {
        r"
1721
979
366
299
675
1456"
            .split('\n')
            .skip(1)
            .map(str::to_string)
            .collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(514579, Solution::new(example_inputs()).part_1());
    }

    #[test]
    fn example_2() {
        assert_eq!(241861950, Solution::new(example_inputs()).part_2());
    }
}
