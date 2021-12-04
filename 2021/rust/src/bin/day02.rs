use std::io::{BufRead, BufReader};

enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

struct Solution {
    commands: Vec<Command>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        let parse = |s: &String| {
            let i = s.find(' ').unwrap();
            let units = s[i + 1..].parse().unwrap();
            match &s[..i] {
                "forward" => Command::Forward(units),
                "down" => Command::Down(units),
                "up" => Command::Up(units),
                _ => unreachable!(),
            }
        };
        Self {
            commands: inputs.iter().map(parse).collect(),
        }
    }
    fn part_1(&self) -> u32 {
        let position = self.calculate_position();
        position.0 * position.2
    }
    fn part_2(&self) -> u32 {
        let position = self.calculate_position();
        position.0 * position.1
    }
    fn calculate_position(&self) -> (u32, u32, u32) {
        self.commands.iter().fold((0, 0, 0), |acc, c| match c {
            Command::Forward(u) => (acc.0 + u, acc.1 + u * acc.2, acc.2),
            Command::Down(n) => (acc.0, acc.1, acc.2 + n),
            Command::Up(n) => (acc.0, acc.1, acc.2 - n),
        })
    }
}

fn main() {
    let solution = Solution::new(
        &BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(Result::ok)
            .collect::<Vec<_>>(),
    );
    println!("{}", solution.part_1());
    println!("{}", solution.part_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_inputs() -> Vec<String> {
        r"
forward 5
down 5
forward 8
up 3
down 8
forward 2"[1..]
            .split('\n')
            .map(String::from)
            .collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(150, Solution::new(&example_inputs()).part_1());
    }

    #[test]
    fn example_2() {
        assert_eq!(900, Solution::new(&example_inputs()).part_2());
    }
}