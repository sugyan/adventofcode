use aoc2022::Solve;
use std::io::{BufRead, BufReader, Read};

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

struct Solution {
    program: Vec<Instruction>,
}

impl Solve for Solution {
    type Answer1 = i32;
    type Answer2 = i32;

    fn new(r: impl Read) -> Self {
        Self {
            program: BufReader::new(r)
                .lines()
                .filter_map(Result::ok)
                .map(|line| match &line[..4] {
                    "noop" => Instruction::Noop,
                    "addx" => Instruction::Addx(line[5..].parse().unwrap()),
                    _ => unreachable!(),
                })
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let mut iter = self.program.iter();
        let (mut x, mut adding) = (1, None);
        let mut sum = 0;
        for i in 1.. {
            if matches!(i, 20 | 60 | 100 | 140 | 180 | 220) {
                sum += i * x;
            }
            adding = if let Some(n) = adding {
                x += n;
                None
            } else {
                match iter.next() {
                    Some(Instruction::Noop) => None,
                    Some(Instruction::Addx(n)) => Some(n),
                    None => break,
                }
            };
        }
        sum
    }
    fn part2(&self) -> Self::Answer2 {
        todo!()
    }
}

fn main() {
    let solution = Solution::new(std::io::stdin().lock());
    println!("Part 1: {}", solution.part1());
    println!("Part 2: {}", solution.part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static [u8] {
        r"
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(13140, Solution::new(example_input()).part1());
    }
}
