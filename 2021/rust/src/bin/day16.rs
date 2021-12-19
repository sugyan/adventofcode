use itertools::Itertools;
use std::io::{BufRead, BufReader};

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
        let (packet, _) = Self::parse(&self.bits);
        packet.total_version_numbers()
    }
    fn part_2(&self) -> u64 {
        let (packet, _) = Self::parse(&self.bits);
        packet.calculate_value()
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

fn main() {
    let solution = Solution::new(
        &BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(Result::ok)
            .collect::<Vec<_>>(),
    );
    println!("{}", solution.part_1());
    println!("{}", solution.part_2());
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

    #[test]
    fn example_2() {
        assert_eq!(3, Solution::new(&[String::from("C200B40A82")]).part_2());
        assert_eq!(54, Solution::new(&[String::from("04005AC33890")]).part_2());
        assert_eq!(7, Solution::new(&[String::from("880086C3E88112")]).part_2());
        assert_eq!(9, Solution::new(&[String::from("CE00C43D881120")]).part_2());
        assert_eq!(1, Solution::new(&[String::from("D8005AC2A8F0")]).part_2());
        assert_eq!(0, Solution::new(&[String::from("F600BC2D8F")]).part_2());
        assert_eq!(0, Solution::new(&[String::from("9C005AC2F8F0")]).part_2());
        assert_eq!(
            1,
            Solution::new(&[String::from("9C0141080250320F1802104A08")]).part_2()
        );
    }
}
