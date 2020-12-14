use std::io::{BufRead, BufReader};

struct Solution {
    instructions: Vec<(String, i32)>,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        Self {
            instructions: inputs
                .iter()
                .map(|input| (input[..3].to_string(), input[4..].parse().unwrap()))
                .collect(),
        }
    }
    fn solve_1(&self) -> i32 {
        self.run(None).unwrap_err()
    }
    fn solve_2(&self) -> i32 {
        for (i, instruction) in self.instructions.iter().enumerate() {
            if instruction.0 != "acc" {
                if let Ok(n) = self.run(Some(i as i32)) {
                    return n;
                }
            }
        }
        0
    }
    fn run(&self, change: Option<i32>) -> Result<i32, i32> {
        let mut visited: Vec<bool> = vec![false; self.instructions.len()];
        let (mut i, mut acc) = (0, 0);
        while i < self.instructions.len() as i32 {
            if visited[i as usize] {
                return Err(acc);
            }
            visited[i as usize] = true;
            let instruction = &self.instructions[i as usize];
            match instruction.0.as_str() {
                "acc" => acc += instruction.1,
                "jmp" if change != Some(i) => i += instruction.1 - 1,
                "nop" if change == Some(i) => i += instruction.1 - 1,
                _ => {}
            }
            i += 1;
        }
        Ok(acc)
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
            5,
            Solution::new(
                "
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"[1..]
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
            8,
            Solution::new(
                "
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"[1..]
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect()
            )
            .solve_2()
        );
    }
}
