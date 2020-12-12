use std::io::{BufRead, BufReader};

struct Solution {
    inputs: Vec<String>,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        Self { inputs }
    }
    fn solve_1(&self) -> i32 {
        let mut p = (0, 0);
        let mut d = (1, 0);
        for input in self.inputs.iter() {
            if let Ok(value) = input[1..].parse::<i32>() {
                match input.chars().next() {
                    Some('N') => p.1 += value,
                    Some('S') => p.1 -= value,
                    Some('E') => p.0 += value,
                    Some('W') => p.0 -= value,
                    Some('L') => {
                        d = match value {
                            90 => (-d.1, d.0),
                            180 => (-d.0, -d.1),
                            270 => (d.1, -d.0),
                            _ => d,
                        }
                    }
                    Some('R') => {
                        d = match value {
                            90 => (d.1, -d.0),
                            180 => (-d.0, -d.1),
                            270 => (-d.1, d.0),
                            _ => d,
                        }
                    }
                    Some('F') => p = (p.0 + d.0 * value, p.1 + d.1 * value),
                    _ => {}
                }
            }
        }
        p.0.abs() + p.1.abs()
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
            25,
            Solution::new(
                "
F10
N3
F7
R90
F11"[1..]
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect()
            )
            .solve_1()
        );
    }
}
