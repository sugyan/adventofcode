use aoc2021::Solve;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    timers: Vec<u8>,
}

impl Solution {
    fn count_lanternfishes(&self, days: usize) -> u64 {
        let mut counts = [0; 9];
        for &t in &self.timers {
            counts[t as usize] += 1;
        }
        for _ in 0..days {
            counts.rotate_left(1);
            counts[6] += counts[8];
        }
        counts.iter().sum()
    }
}

impl Solve for Solution {
    type Answer1 = u64;
    type Answer2 = u64;

    fn new(r: impl Read) -> Self {
        Self {
            timers: BufReader::new(r)
                .lines()
                .find_map(Result::ok)
                .unwrap()
                .split(',')
                .filter_map(|x| x.parse().ok())
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.count_lanternfishes(80)
    }
    fn part2(&self) -> Self::Answer2 {
        self.count_lanternfishes(256)
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
3,4,3,1,2
"[1..]
            .as_bytes()
    }

    #[test]
    fn example1() {
        assert_eq!(5934, Solution::new(example_input()).part1());
    }

    #[test]
    fn example2() {
        assert_eq!(26_984_457_539, Solution::new(example_input()).part2());
    }
}
