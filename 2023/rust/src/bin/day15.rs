use aoc2023::Solve;
use std::io::{BufReader, Read};

enum Operation {
    Sign(u32),
    Dash,
}

struct Solution {
    sequence: Vec<String>,
}

impl Solution {
    fn hash(s: impl AsRef<str>) -> usize {
        s.as_ref()
            .bytes()
            .fold(0, |acc, u| (acc + u as usize) * 17 % 256)
    }
}

impl Solve for Solution {
    type Answer1 = usize;
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
        self.sequence.iter().map(Self::hash).sum()
    }
    fn part2(&self) -> Self::Answer2 {
        let mut boxes = vec![Vec::new(); 256];
        for s in &self.sequence {
            let (label, operation) = if let Some((label, focal_length)) = s.split_once('=') {
                (
                    label,
                    Operation::Sign(focal_length.parse().expect("should be valid number")),
                )
            } else {
                (&s[..s.len() - 1], Operation::Dash)
            };
            let index = Self::hash(label);
            match operation {
                Operation::Sign(focal_length) => {
                    if let Some(i) = boxes[index].iter().position(|&(l, _)| l == label) {
                        boxes[index][i].1 = focal_length;
                    } else {
                        boxes[index].push((label, focal_length));
                    }
                }
                Operation::Dash => {
                    boxes[index].retain(|&(l, _)| l != label);
                }
            }
        }
        (1..)
            .zip(boxes)
            .map(|(box_number, b)| {
                (1..)
                    .zip(b)
                    .map(|(slot_number, (_, focal_length))| box_number * slot_number * focal_length)
                    .sum::<u32>()
            })
            .sum()
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

    #[test]
    fn part2() {
        assert_eq!(Solution::new(example_input()).part2(), 145);
    }
}
