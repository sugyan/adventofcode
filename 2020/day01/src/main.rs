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
    fn solve_1(&self) -> i32 {
        let hs = self.reports.iter().collect::<HashSet<_>>();
        for &i in self.reports.iter() {
            if hs.contains(&(2020 - i)) {
                return (2020 - i) * i;
            }
        }
        0
    }
    fn solve_2(&self) -> i32 {
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
        0
    }
}

fn main() {
    let inputs = BufReader::new(std::io::stdin().lock())
        .lines()
        .filter_map(|line| line.ok())
        .collect();
    let solution = Solution::new(inputs);
    println!("{}", solution.solve_1());
    println!("{}", solution.solve_2());
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
        assert_eq!(514579, Solution::new(example_inputs()).solve_1());
    }

    #[test]
    fn example_2() {
        assert_eq!(241861950, Solution::new(example_inputs()).solve_2());
    }
}
