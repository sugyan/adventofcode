use aoc2022::Solve;
use itertools::Itertools;
use std::cmp::Ordering;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

#[derive(PartialEq, Eq)]
enum Value {
    List(Vec<Value>),
    Integer(u8),
}

impl Value {
    fn as_slice(&self) -> &[Value] {
        if let Self::List(v) = self {
            v.as_slice()
        } else {
            std::slice::from_ref(self)
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        if let (Self::Integer(lhs), Self::Integer(rhs)) = (self, other) {
            lhs.cmp(rhs)
        } else {
            self.as_slice().cmp(other.as_slice())
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Packet(Value);

impl FromStr for Packet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_list(iter: &mut impl Iterator<Item = u8>) -> Value {
            let mut v = Vec::new();
            while let Some(u) = iter.next() {
                match u {
                    b'[' => v.push(parse_list(iter)),
                    b']' => break,
                    b'0'..=b':' => v.push(Value::Integer(u - b'0')),
                    _ => {}
                }
            }
            Value::List(v)
        }
        Ok(Self(parse_list(&mut s.replace("10", ":").bytes())))
    }
}

struct Solution {
    pairs: Vec<(Packet, Packet)>,
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;

    fn new(r: impl Read) -> Self {
        Self {
            pairs: BufReader::new(r)
                .lines()
                .filter_map(Result::ok)
                .collect::<Vec<_>>()
                .split(String::is_empty)
                .filter_map(|lines| lines.iter().filter_map(|s| s.parse().ok()).collect_tuple())
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.pairs
            .iter()
            .enumerate()
            .filter_map(
                |(i, (left, right))| {
                    if left < right {
                        Some(i + 1)
                    } else {
                        None
                    }
                },
            )
            .sum()
    }
    fn part2(&self) -> Self::Answer2 {
        let divider_packets = ["[[2]]", "[[6]]"]
            .iter()
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<_>>();
        let mut packets = self
            .pairs
            .iter()
            .flat_map(|pair| [&pair.0, &pair.1])
            .chain(&divider_packets)
            .collect::<Vec<_>>();
        packets.sort();
        divider_packets
            .iter()
            .filter_map(|packet| packets.binary_search(&packet).ok())
            .map(|i| i + 1)
            .product()
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
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(13, Solution::new(example_input()).part1());
    }

    #[test]
    fn part2() {
        assert_eq!(140, Solution::new(example_input()).part2());
    }
}
