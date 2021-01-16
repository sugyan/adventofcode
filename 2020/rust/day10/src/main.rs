use std::io::{BufRead, BufReader};

struct Solution {
    adapters: Vec<u8>,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        let mut adapters = inputs
            .iter()
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<_>>();
        adapters.push(0);
        adapters.sort_unstable();
        Self { adapters }
    }
    fn part_1(&self) -> u64 {
        let (mut diff1, mut diff3) = (0, 0);
        for i in 1..self.adapters.len() {
            match self.adapters[i] - self.adapters[i - 1] {
                1 => diff1 += 1,
                3 => diff3 += 1,
                _ => {}
            }
        }
        diff1 * (diff3 + 1)
    }
    fn part_2(&self) -> u64 {
        let mut dp = vec![0; self.adapters.len()];
        dp[0] = 1;
        for i in 1..dp.len() {
            for j in 1..=3 {
                if i >= j && self.adapters[i] - self.adapters[i - j] <= 3 {
                    dp[i] += dp[i - j];
                }
            }
        }
        dp[self.adapters.len() - 1]
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

    fn example_inputs_1() -> Vec<String> {
        r"
16
10
15
5
1
11
7
19
6
12
4"
        .split('\n')
        .skip(1)
        .map(str::to_string)
        .collect()
    }

    fn example_inputs_2() -> Vec<String> {
        r"
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"
        .split('\n')
        .skip(1)
        .map(str::to_string)
        .collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(35, Solution::new(example_inputs_1()).part_1());
        assert_eq!(220, Solution::new(example_inputs_2()).part_1());
    }

    #[test]
    fn example_2() {
        assert_eq!(8, Solution::new(example_inputs_1()).part_2());
        assert_eq!(19208, Solution::new(example_inputs_2()).part_2());
    }
}
