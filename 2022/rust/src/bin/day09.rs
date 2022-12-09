use aoc2022::Solve;
use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read};

#[derive(Debug)]
enum Motion {
    R(u32),
    U(u32),
    L(u32),
    D(u32),
}

impl From<String> for Motion {
    fn from(s: String) -> Self {
        let n = s[2..].parse().unwrap();
        match &s[..1] {
            "R" => Self::R(n),
            "U" => Self::U(n),
            "L" => Self::L(n),
            "D" => Self::D(n),
            _ => unreachable!(),
        }
    }
}

struct Solution {
    motions: Vec<Motion>,
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;

    fn new(r: impl Read) -> Self {
        Self {
            motions: BufReader::new(r)
                .lines()
                .filter_map(Result::ok)
                .map(Motion::from)
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let (mut head, mut tail) = ((0_i32, 0_i32), (0_i32, 0_i32));
        let mut hs = HashSet::from([tail]);
        for motion in &self.motions {
            let (d, n) = match motion {
                Motion::R(n) => ((1, 0), n),
                Motion::U(n) => ((0, 1), n),
                Motion::L(n) => ((-1, 0), n),
                Motion::D(n) => ((0, -1), n),
            };
            for _ in 0..*n {
                head.0 += d.0;
                head.1 += d.1;
                let diff = (head.0 - tail.0, head.1 - tail.1);
                if diff.0.abs() > 1 || diff.1.abs() > 1 {
                    tail.0 += diff.0.signum();
                    tail.1 += diff.1.signum();
                }
                hs.insert(tail);
            }
        }
        hs.len()
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
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(13, Solution::new(example_input()).part1());
    }
}
