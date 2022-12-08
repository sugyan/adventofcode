use aoc2022::Solve;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    counts: [u32; 9],
}

impl Solution {
    fn total_score(&self, score_map: [u32; 9]) -> u32 {
        self.counts.iter().zip(&score_map).map(|(c, s)| c * s).sum()
    }
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        let mut counts = [0; 9];
        BufReader::new(r)
            .lines()
            .filter_map(Result::ok)
            .for_each(|s| {
                let b = s.as_bytes();
                counts[((b[0] - b'A') * 3 + b[2] - b'X') as usize] += 1;
            });
        Self { counts }
    }
    fn part1(&self) -> Self::Answer1 {
        // `A X`: 1 + 3 => 4, `A Y`: 2 + 6 => 8, `A Z`: 3 + 0 => 3,
        // `B X`: 1 + 0 => 1, `B Y`: 2 + 3 => 5, `B Z`: 3 + 6 => 9,
        // `C X`: 1 + 6 => 7, `C Y`: 2 + 0 => 2, `C Z`: 3 + 3 => 6,
        self.total_score([4, 8, 3, 1, 5, 9, 7, 2, 6])
    }
    fn part2(&self) -> Self::Answer2 {
        // `A X`: 3 + 0 => 3, `A Y`: 1 + 3 => 4, `A Z`: 2 + 6 => 8,
        // `B X`: 1 + 0 => 1, `B Y`: 2 + 3 => 5, `B Z`: 3 + 6 => 9,
        // `C X`: 2 + 0 => 2, `C Y`: 3 + 3 => 6, `C Z`: 1 + 6 => 7,
        self.total_score([3, 4, 8, 1, 5, 9, 2, 6, 7])
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
A Y
B X
C Z
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(15, Solution::new(example_input()).part1());
    }

    #[test]
    fn part2() {
        assert_eq!(12, Solution::new(example_input()).part2());
    }
}
