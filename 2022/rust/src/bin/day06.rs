use aoc2022::Solve;
use std::io::{BufReader, Read};

struct Solution {
    data: Vec<u8>,
}

impl Solution {
    fn marker_detected_position(&self, window_size: usize) -> usize {
        self.data
            .windows(window_size)
            .position(|w| {
                w.iter().fold(0_u128, |acc, x| acc | 1 << x).count_ones() as usize == window_size
            })
            .unwrap()
            + window_size
    }
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;

    fn new(r: impl Read) -> Self {
        let mut data = Vec::new();
        BufReader::new(r).read_to_end(&mut data).unwrap();
        Self { data }
    }
    fn part1(&self) -> Self::Answer1 {
        self.marker_detected_position(4)
    }
    fn part2(&self) -> Self::Answer2 {
        self.marker_detected_position(14)
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

    #[test]
    fn part2() {
        assert_eq!(
            19,
            Solution::new("mjqjpqmgbljsphdztnvjfqwrcgsmlb".as_bytes()).part2()
        );
        assert_eq!(
            23,
            Solution::new("bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes()).part2()
        );
        assert_eq!(
            23,
            Solution::new("nppdvjthqldpwncqszvftbrmjlhg".as_bytes()).part2()
        );
        assert_eq!(
            29,
            Solution::new("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes()).part2()
        );
        assert_eq!(
            26,
            Solution::new("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes()).part2()
        );
    }
}
