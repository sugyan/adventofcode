use std::io::{BufRead, BufReader};

struct Solution {
    inputs: Vec<String>,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        Self { inputs }
    }
    fn solve_1(&self) -> i32 {
        self.run(None).unwrap_err()
    }
    fn solve_2(&self) -> i32 {
        for (i, input) in self.inputs.iter().enumerate() {
            if !input.starts_with("acc") {
                if let Ok(n) = self.run(Some(i as i32)) {
                    return n;
                }
            }
        }
        0
    }
    fn run(&self, change: Option<i32>) -> Result<i32, i32> {
        let mut visited: Vec<bool> = vec![false; self.inputs.len()];
        let (mut i, mut acc) = (0, 0);
        while i < self.inputs.len() as i32 {
            if visited[i as usize] {
                return Err(acc);
            }
            visited[i as usize] = true;
            let changed = Some(i) == change;
            let instruction = &self.inputs[i as usize];
            if let Ok(arg) = instruction[4..].parse::<i32>() {
                match &instruction[..3] {
                    "acc" => acc += arg,
                    "jmp" => {
                        if !changed {
                            i += arg - 1
                        }
                    }
                    "nop" => {
                        if changed {
                            i += arg - 1
                        }
                    }
                    _ => {}
                }
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
