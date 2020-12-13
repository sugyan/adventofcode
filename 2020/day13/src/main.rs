use std::io::{BufRead, BufReader};

struct Solution {
    inputs: Vec<String>,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        Self { inputs }
    }
    fn solve_1(&self) -> i32 {
        let timestamp = self.inputs[0].parse::<i32>().unwrap();
        if let Some((minutes, id)) = self.inputs[1]
            .split(',')
            .filter_map(|s| s.parse::<i32>().ok())
            .map(|id| (id * ((timestamp - 1) / id + 1) - timestamp, id))
            .min_by_key(|&e| e.0)
        {
            id * minutes
        } else {
            0
        }
    }
}

fn main() {
    let solution = Solution::new(
        BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(|line| line.ok())
            .collect(),
    );
    println!("{}", solution.solve_1());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            295,
            Solution::new(
                "
939
7,13,x,x,59,x,31,19"[1..]
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect()
            )
            .solve_1()
        );
    }
}
