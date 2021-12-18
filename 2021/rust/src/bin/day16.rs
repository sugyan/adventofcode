use itertools::Itertools;
use std::io::{BufRead, BufReader};

enum Value {
    Literal(u32),
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
}

struct Solution {
    bits: String,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        Self {
            bits: inputs[0]
                .chars()
                .map(|c| format!("{:04b}", u8::from_str_radix(&c.to_string(), 16).unwrap()))
                .join(""),
        }
    }
    fn part_1(&self) -> u32 {
        let mut bits = self.bits.as_str();
        let mut total = 0;
        while bits.bytes().any(|u| u > b'0') {
            let (packet, s) = Self::parse(bits);
            bits = s;
            total += packet.total_version_numbers();
        }
        total
    }
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
                value += group & 0x0f;
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

fn main() {
    let solution = Solution::new(
        &BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(Result::ok)
            .collect::<Vec<_>>(),
    );
    println!("{}", solution.part_1());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            16,
            Solution::new(&[String::from("8A004A801A8002F478")]).part_1()
        );
        assert_eq!(
            12,
            Solution::new(&[String::from("620080001611562C8802118E34")]).part_1()
        );
        assert_eq!(
            23,
            Solution::new(&[String::from("C0015000016115A2E0802F182340")]).part_1()
        );
        assert_eq!(
            31,
            Solution::new(&[String::from("A0016C880162017C3686B18A3D4780")]).part_1()
        );
    }
}
