use std::io::{BufRead, BufReader};

struct Solution {
    instructions: Vec<(char, i32)>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        Self {
            instructions: inputs
                .iter()
                .filter_map(|input| {
                    if let (Some(action), Ok(value)) = (input.chars().next(), input[1..].parse()) {
                        Some((action, value))
                    } else {
                        None
                    }
                })
                .collect(),
        }
    }
    fn part_1(&self) -> i32 {
        self.run((1, 0), false)
    }
    fn part_2(&self) -> i32 {
        self.run((10, 1), true)
    }
    fn run(&self, w: (i32, i32), waypoint: bool) -> i32 {
        let mut p = (0, 0);
        let mut w = w;
        for &(action, value) in &self.instructions {
            match action {
                'N' if waypoint => w.1 += value,
                'N' => p.1 += value,
                'S' if waypoint => w.1 -= value,
                'S' => p.1 -= value,
                'E' if waypoint => w.0 += value,
                'E' => p.0 += value,
                'W' if waypoint => w.0 -= value,
                'W' => p.0 -= value,
                'L' => {
                    for _ in 0..value / 90 {
                        w = (-w.1, w.0)
                    }
                }
                'R' => {
                    for _ in 0..value / 90 {
                        w = (w.1, -w.0)
                    }
                }
                'F' => p = (p.0 + w.0 * value, p.1 + w.1 * value),
                _ => {}
            }
        }
        p.0.abs() + p.1.abs()
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
F10
N3
F7
R90
F11"
        .split('\n')
        .skip(1)
        .map(str::to_string)
        .collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(25, Solution::new(&example_inputs()).part_1());
    }

    #[test]
    fn example_2() {
        assert_eq!(286, Solution::new(&example_inputs()).part_2());
    }
}
