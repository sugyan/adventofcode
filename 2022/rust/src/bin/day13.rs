use aoc2022::Solve;
use itertools::Itertools;
use std::cmp::Ordering;
use std::io::{BufRead, BufReader, Read};

#[derive(Debug, Clone, PartialEq)]
enum Value {
    Integer(u8),
    List(Vec<Value>),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Integer(lhs), Value::Integer(rhs)) => Some(lhs.cmp(rhs)),
            (Value::List(lhs), Value::List(rhs)) => {
                let (mut l, mut r) = (lhs.iter(), rhs.iter());
                loop {
                    match (l.next(), r.next()) {
                        (Some(lhs), Some(rhs)) => match lhs.partial_cmp(rhs) {
                            Some(Ordering::Equal) => continue,
                            o => break o,
                        },
                        (Some(_), None) => break Some(Ordering::Greater),
                        (None, Some(_)) => break Some(Ordering::Less),
                        (None, None) => break Some(Ordering::Equal),
                    }
                }
            }
            (Value::Integer(n), _) => Value::List(vec![Value::Integer(*n)]).partial_cmp(other),
            (_, Value::Integer(n)) => self.partial_cmp(&Value::List(vec![Value::Integer(*n)])),
        }
    }
}

#[derive(Debug, Clone)]
struct Packet(Vec<Value>);

impl From<&String> for Packet {
    fn from(s: &String) -> Self {
        fn parse(iter: &mut impl Iterator<Item = u8>) -> Vec<Value> {
            let mut v = Vec::new();
            let mut n = None;
            while let Some(u) = iter.next() {
                match u {
                    b'[' => v.push(Value::List(parse(iter))),
                    b']' => break,
                    b'0'..=b'9' => n = Some(n.map_or(u - b'0', |n| n * 10 + u - b'0')),
                    _ => {
                        if let Some(n) = n {
                            v.push(Value::Integer(n));
                        }
                        n = None;
                    }
                }
            }
            if let Some(n) = n {
                v.push(Value::Integer(n));
            }
            v
        }
        Self(parse(&mut s.bytes()))
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
                .filter_map(|lines| lines.iter().map(Packet::from).collect_tuple())
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.pairs
            .iter()
            .enumerate()
            .filter_map(
                |(i, (left, right))| {
                    if left.0 < right.0 {
                        Some(i + 1)
                    } else {
                        None
                    }
                },
            )
            .sum()
    }
    fn part2(&self) -> Self::Answer2 {
        let divider_packets = [
            Packet(vec![Value::List(vec![Value::List(vec![Value::Integer(
                2,
            )])])]),
            Packet(vec![Value::List(vec![Value::List(vec![Value::Integer(
                6,
            )])])]),
        ];
        let mut packets = self
            .pairs
            .iter()
            .flat_map(|pair| vec![pair.0.clone(), pair.1.clone()])
            .chain(divider_packets.iter().cloned())
            .collect::<Vec<_>>();
        packets.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        divider_packets
            .iter()
            .filter_map(|packet| packets.iter().position(|p| p.0 == packet.0))
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
    fn test_part1() {
        assert_eq!(13, Solution::new(example_input()).part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(140, Solution::new(example_input()).part2());
    }
}
