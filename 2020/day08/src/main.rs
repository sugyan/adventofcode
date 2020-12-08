use std::io::{BufRead, BufReader};

struct Solution {
    inputs: Vec<String>,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        Self { inputs }
    }
    fn solve_1(&self) -> i32 {
        let mut visited: Vec<bool> = vec![false; self.inputs.len()];
        let (mut i, mut acc) = (0, 0);
        loop {
            if visited[i as usize] {
                return acc;
            }
            visited[i as usize] = true;
            let instruction = &self.inputs[i as usize];
            if let Ok(arg) = instruction[4..].parse::<i32>() {
                match &instruction[..3] {
                    "acc" => acc += arg,
                    "jmp" => i += arg - 1,
                    _ => {}
                }
            }
            i += 1;
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
}
