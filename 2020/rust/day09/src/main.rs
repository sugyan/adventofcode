use std::collections::VecDeque;
use std::io::{BufRead, BufReader};

struct Solution {
    data: Vec<u64>,
    preamble: usize,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        Self {
            data: inputs.iter().filter_map(|s| s.parse().ok()).collect(),
            preamble: 25,
        }
    }
    #[cfg(test)]
    fn set_preamble(&mut self, preamble: usize) {
        self.preamble = preamble;
    }
    fn part_1(&self) -> u64 {
        let mut vd = self.data.iter().take(self.preamble).collect();
        let has_pair = |vd: &VecDeque<&u64>, target: u64| -> bool {
            for &n1 in vd.iter() {
                for &n2 in vd.iter() {
                    if *n1 != *n2 && *n1 + *n2 == target {
                        return true;
                    }
                }
            }
            false
        };
        for i in self.preamble..self.data.len() {
            if !has_pair(&vd, self.data[i]) {
                return self.data[i];
            }
            vd.pop_front();
            vd.push_back(&self.data[i]);
        }
        unreachable!()
    }
    fn part_2(&self) -> u64 {
        let target = self.part_1();
        let (mut l, mut r) = (0, 0);
        let mut sum = self.data[0];
        while sum != target {
            if sum < target {
                r += 1;
                sum += self.data[r];
            } else {
                sum -= self.data[l];
                l += 1;
            }
        }
        let (mut min, mut max) = (std::u64::MAX, std::u64::MIN);
        for i in l..=r {
            min = std::cmp::min(min, self.data[i]);
            max = std::cmp::max(max, self.data[i]);
        }
        min + max
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
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"
        .split('\n')
        .skip(1)
        .map(str::to_string)
        .collect()
    }

    #[test]
    fn example_1() {
        let mut solution = Solution::new(&example_inputs());
        solution.set_preamble(5);
        assert_eq!(127, solution.part_1());
    }

    #[test]
    fn example_2() {
        let mut solution = Solution::new(&example_inputs());
        solution.set_preamble(5);
        assert_eq!(62, solution.part_2());
    }
}
