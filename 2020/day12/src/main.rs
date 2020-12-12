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
        let mut p = (0, 0);
        let mut d = (1, 0);
        for &instruction in self.instructions.iter() {
            match instruction.0 {
                'N' => p.1 += instruction.1,
                'S' => p.1 -= instruction.1,
                'E' => p.0 += instruction.1,
                'W' => p.0 -= instruction.1,
                'L' => {
                    d = match instruction.1 {
                        90 => (-d.1, d.0),
                        180 => (-d.0, -d.1),
                        270 => (d.1, -d.0),
                        _ => d,
                    }
                }
                'R' => {
                    d = match instruction.1 {
                        90 => (d.1, -d.0),
                        180 => (-d.0, -d.1),
                        270 => (-d.1, d.0),
                        _ => d,
                    }
                }
                'F' => p = (p.0 + d.0 * instruction.1, p.1 + d.1 * instruction.1),
                _ => {}
            }
        }
        p.0.abs() + p.1.abs()
    }
    fn solve_2(&self) -> i32 {
        let mut p = (0, 0);
        let mut w = (10, 1);
        for &instruction in self.instructions.iter() {
            match instruction.0 {
                'N' => w.1 += instruction.1,
                'S' => w.1 -= instruction.1,
                'E' => w.0 += instruction.1,
                'W' => w.0 -= instruction.1,
                'L' => {
                    w = match instruction.1 {
                        90 => (-w.1, w.0),
                        180 => (-w.0, -w.1),
                        270 => (w.1, -w.0),
                        _ => w,
                    }
                }
                'R' => {
                    w = match instruction.1 {
                        90 => (w.1, -w.0),
                        180 => (-w.0, -w.1),
                        270 => (-w.1, w.0),
                        _ => w,
                    }
                }
                'F' => p = (p.0 + w.0 * instruction.1, p.1 + w.1 * instruction.1),
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
