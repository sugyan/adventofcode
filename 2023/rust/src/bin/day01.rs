use aoc2023::Solve;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    input: Vec<String>,
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        Self {
            input: BufReader::new(r).lines().map_while(Result::ok).collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let parse = |s: &String| {
            let v = s
                .chars()
                .filter(char::is_ascii_digit)
                .map(|c| u32::from((c as u8) - b'0'))
                .collect::<Vec<_>>();
            v[0] * 10 + v[v.len() - 1]
        };
        self.input.iter().map(parse).sum()
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
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(142, Solution::new(example_input()).part1());
    }
}
