use aoc2022::Solve;
use std::io::{BufRead, BufReader, Read};

enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

#[derive(Default)]
struct Position(u32, u32, u32);

struct Solution {
    commands: Vec<Command>,
}

impl Solution {
    fn calculate_position(&self) -> Position {
        self.commands
            .iter()
            .fold(Position::default(), |acc, c| match c {
                Command::Forward(u) => Position(acc.0 + u, acc.1 + u * acc.2, acc.2),
                Command::Down(u) => Position(acc.0, acc.1, acc.2 + u),
                Command::Up(u) => Position(acc.0, acc.1, acc.2 - u),
            })
    }
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        let parse = |s: String| {
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
            commands: BufReader::new(r)
                .lines()
                .filter_map(Result::ok)
                .map(parse)
                .collect(),
        }
    }
    fn part1(&self) -> u32 {
        let position = self.calculate_position();
        position.0 * position.2
    }
    fn part2(&self) -> u32 {
        let position = self.calculate_position();
        position.0 * position.1
    }
}

fn main() {
    let solution = Solution::new(std::io::stdin().lock());
    println!("{}", solution.part1());
    println!("{}", solution.part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static [u8] {
        r"
forward 5
down 5
forward 8
up 3
down 8
forward 2"[1..]
            .as_bytes()
    }

    #[test]
    fn example_1() {
        assert_eq!(150, Solution::new(example_input()).part1());
    }

    #[test]
    fn example_2() {
        assert_eq!(900, Solution::new(example_input()).part2());
    }
}
