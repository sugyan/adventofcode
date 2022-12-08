use aoc2021::Solve;
use itertools::Itertools;
use std::io::{BufRead, BufReader, Read};

enum Value {
    Literal(u64),
    Operator(Vec<Packet>),
}

struct Packet {
    version: u32,
    type_id: u32,
    value: Value,
}

impl Packet {
    fn total_version_numbers(&self) -> u32 {
        self.version
            + match &self.value {
                Value::Literal(_) => 0,
                Value::Operator(packets) => packets.iter().map(|p| p.total_version_numbers()).sum(),
            }
    }
    fn calculate_value(&self) -> u64 {
        match &self.value {
            Value::Literal(u) => *u,
            Value::Operator(packets) => {
                let values = packets
                    .iter()
                    .map(|p| p.calculate_value())
                    .collect::<Vec<_>>();
                match self.type_id {
                    0 => values.iter().sum(),
                    1 => values.iter().product(),
                    2 => *values.iter().min().unwrap(),
                    3 => *values.iter().max().unwrap(),
                    5 | 6 | 7 if values.len() != 2 => panic!("Invalid packet length"),
                    5 if values[0] > values[1] => 1,
                    6 if values[0] < values[1] => 1,
                    7 if values[0] == values[1] => 1,
                    5 | 6 | 7 => 0,
                    _ => unreachable!(),
                }
            }
        }
    }
}

struct Solution {
    packet: Packet,
}

impl Solution {
    fn parse(s: &str) -> (Packet, &str) {
        let (version, s) = Self::get_value(s, 3);
        let (type_id, s) = Self::get_value(s, 3);
        if type_id == 4 {
            let mut s = s;
            let mut value = 0;
            loop {
                let (group, ss) = Self::get_value(s, 5);
                s = ss;
                value <<= 4;
                value += (group & 0x0f) as u64;
                if group & 0x10 == 0 {
                    break;
                }
            }
            return (
                Packet {
                    version,
                    type_id,
                    value: Value::Literal(value),
                },
                s,
            );
        }
        let (length_type_id, s) = Self::get_value(s, 1);
        let (sub_packets, s) = if length_type_id == 0 {
            let (length, s) = Self::get_value(s, 15);
            let mut sub = &s[..length as usize];
            let mut sub_packets = Vec::new();
            while !sub.is_empty() {
                let (packet, s) = Self::parse(sub);
                sub_packets.push(packet);
                sub = s;
            }
            (sub_packets, &s[length as usize..])
        } else {
            let (length, s) = Self::get_value(s, 11);
            let mut sub = s;
            (
                (0..length)
                    .map(|_| {
                        let (packet, s) = Self::parse(sub);
                        sub = s;
                        packet
                    })
                    .collect(),
                sub,
            )
        };
        (
            Packet {
                version,
                type_id,
                value: Value::Operator(sub_packets),
            },
            s,
        )
    }
    fn get_value(s: &str, len: usize) -> (u32, &str) {
        (u32::from_str_radix(&s[0..len], 2).unwrap(), &s[len..])
    }
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u64;

    fn new(r: impl Read) -> Self {
        let (packet, _) = Self::parse(
            &BufReader::new(r)
                .lines()
                .find_map(Result::ok)
                .unwrap()
                .chars()
                .map(|c| format!("{:04b}", u8::from_str_radix(&c.to_string(), 16).unwrap()))
                .join(""),
        );
        Self { packet }
    }
    fn part1(&self) -> Self::Answer1 {
        self.packet.total_version_numbers()
    }
    fn part2(&self) -> Self::Answer2 {
        self.packet.calculate_value()
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

    #[test]
    fn part1() {
        assert_eq!(16, Solution::new("8A004A801A8002F478".as_bytes()).part1());
        assert_eq!(
            12,
            Solution::new("620080001611562C8802118E34".as_bytes()).part1()
        );
        assert_eq!(
            23,
            Solution::new("C0015000016115A2E0802F182340".as_bytes()).part1()
        );
        assert_eq!(
            31,
            Solution::new("A0016C880162017C3686B18A3D4780".as_bytes()).part1()
        );
    }

    #[test]
    fn part2() {
        assert_eq!(3, Solution::new("C200B40A82".as_bytes()).part2());
        assert_eq!(54, Solution::new("04005AC33890".as_bytes()).part2());
        assert_eq!(7, Solution::new("880086C3E88112".as_bytes()).part2());
        assert_eq!(9, Solution::new("CE00C43D881120".as_bytes()).part2());
        assert_eq!(1, Solution::new("D8005AC2A8F0".as_bytes()).part2());
        assert_eq!(0, Solution::new("F600BC2D8F".as_bytes()).part2());
        assert_eq!(0, Solution::new("9C005AC2F8F0".as_bytes()).part2());
        assert_eq!(
            1,
            Solution::new("9C0141080250320F1802104A08".as_bytes()).part2()
        );
    }
}
