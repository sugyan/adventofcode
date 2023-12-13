use aoc2023::Solve;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    patterns: Vec<Vec<Vec<bool>>>,
}

impl Solution {
    fn summarized_number(pattern: &[Vec<bool>], target_error: usize) -> u32 {
        let (rows, cols) = (pattern.len(), pattern[0].len());
        let mut ret = 0;
        for (i, num) in (0..rows - 1).zip(1..) {
            if (0..cols)
                .map(|j| {
                    (0..(i + 1).min(rows - i - 1))
                        .filter(|k| pattern[i - k][j] != pattern[i + k + 1][j])
                        .count()
                })
                .sum::<usize>()
                == target_error
            {
                ret += num * 100;
            }
        }
        for (j, num) in (0..cols - 1).zip(1..) {
            if (0..rows)
                .map(|i| {
                    (0..(j + 1).min(cols - j - 1))
                        .filter(|k| pattern[i][j - k] != pattern[i][j + k + 1])
                        .count()
                })
                .sum::<usize>()
                == target_error
            {
                ret += num;
            }
        }
        ret
    }
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        Self {
            patterns: BufReader::new(r)
                .lines()
                .map_while(Result::ok)
                .collect::<Vec<_>>()
                .split(String::is_empty)
                .map(|lines| {
                    lines
                        .iter()
                        .map(|line| line.chars().map(|c| c == '#').collect())
                        .collect()
                })
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.patterns
            .iter()
            .map(|pattern| Self::summarized_number(pattern, 0))
            .sum()
    }
    fn part2(&self) -> Self::Answer2 {
        self.patterns
            .iter()
            .map(|pattern| Self::summarized_number(pattern, 1))
            .sum()
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
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(Solution::new(example_input()).part1(), 405);
    }

    #[test]
    fn part2() {
        assert_eq!(Solution::new(example_input()).part2(), 400);
    }
}
