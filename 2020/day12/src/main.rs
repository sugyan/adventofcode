use std::io::{BufRead, BufReader};

struct Solution {
    instructions: Vec<(char, i32)>,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        Self {
            instructions: inputs
                .iter()
                .filter_map(|input| {
                    if let (Some(action), Ok(value)) =
                        (input.chars().next(), input[1..].parse::<i32>())
                    {
                        Some((action, value))
                    } else {
                        None
                    }
                })
                .collect(),
        }
    }
    fn solve_1(&self) -> i32 {
        self.run((1, 0), false)
    }
    fn solve_2(&self) -> i32 {
        self.run((10, 1), true)
    }
    fn run(&self, w: (i32, i32), waypoint: bool) -> i32 {
        let mut p = (0, 0);
        let mut w = w;
        for &inst in self.instructions.iter() {
            match inst.0 {
                'N' if waypoint => w.1 += inst.1,
                'N' => p.1 += inst.1,
                'S' if waypoint => w.1 -= inst.1,
                'S' => p.1 -= inst.1,
                'E' if waypoint => w.0 += inst.1,
                'E' => p.0 += inst.1,
                'W' if waypoint => w.0 -= inst.1,
                'W' => p.0 -= inst.1,
                'L' => {
                    for _ in 0..inst.1 / 90 {
                        w = (-w.1, w.0)
                    }
                }
                'R' => {
                    for _ in 0..inst.1 / 90 {
                        w = (w.1, -w.0)
                    }
                }
                'F' => p = (p.0 + w.0 * inst.1, p.1 + w.1 * inst.1),
                _ => {}
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
    println!("{}", solution.solve_2());
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

    #[test]
    fn example_2() {
        assert_eq!(
            286,
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
            .solve_2()
        );
    }
}
