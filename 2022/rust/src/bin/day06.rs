use aoc2022::Solve;
use std::io::{BufReader, Read};

struct Solution {
    datastream: Vec<u8>,
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;

    fn new(r: impl Read) -> Self {
        let mut buf = String::new();
        BufReader::new(r).read_to_string(&mut buf).unwrap();
        Self {
            datastream: buf.bytes().collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.datastream
            .windows(4)
            .position(|w| {
                w.iter()
                    .fold(0_u32, |acc, x| acc | 1 << (x - b'a'))
                    .count_ones()
                    == 4
            })
            .unwrap()
            + 4
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

    #[test]
    fn example1() {
        assert_eq!(
            7,
            Solution::new("mjqjpqmgbljsphdztnvjfqwrcgsmlb".as_bytes()).part1()
        );
        assert_eq!(
            5,
            Solution::new("bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes()).part1()
        );
        assert_eq!(
            6,
            Solution::new("nppdvjthqldpwncqszvftbrmjlhg".as_bytes()).part1()
        );
        assert_eq!(
            10,
            Solution::new("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes()).part1()
        );
        assert_eq!(
            11,
            Solution::new("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes()).part1()
        );
    }
}
