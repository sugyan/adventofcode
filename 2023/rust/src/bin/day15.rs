use aoc2023::Solve;
use std::io::{BufReader, Read};

struct Solution {
    sequence: Vec<String>,
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        let mut buf = String::new();
        BufReader::new(r)
            .read_to_string(&mut buf)
            .expect("should be succeeded to read line");
        Self {
            sequence: buf.trim().split(',').map(String::from).collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.sequence
            .iter()
            .map(|s| s.bytes().fold(0, |acc, u| (acc + u as u32) * 17 % 256))
            .sum()
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
        r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(Solution::new(example_input()).part1(), 1320);
    }
}
