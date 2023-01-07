use aoc2022::Solve;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    data: Vec<usize>,
}

impl Solution {
    fn marker_detected_position(&self, window_size: usize) -> usize {
        let mut counts = [0; 26];
        let mut size = 0;
        for i in 0..self.data.len() {
            counts[self.data[i]] += 1;
            if counts[self.data[i]] == 1 {
                size += 1;
            }
            if i >= window_size {
                counts[self.data[i - window_size]] -= 1;
                if counts[self.data[i - window_size]] == 0 {
                    size -= 1;
                }
            }
            if size == window_size {
                return i + 1;
            }
        }
        unreachable!()
    }
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;

    fn new(r: impl Read) -> Self {
        Self {
            data: BufReader::new(r)
                .lines()
                .find_map(Result::ok)
                .map(|s| s.trim().bytes().map(|u| (u - b'a') as usize).collect())
                .unwrap(),
        }
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
