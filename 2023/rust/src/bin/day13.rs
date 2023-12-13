use aoc2023::Solve;
use itertools::Itertools;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    patterns: Vec<Vec<Vec<bool>>>,
}

impl Solution {
    fn summarized_numbers(pattern: &[Vec<bool>]) -> (Vec<u32>, Vec<u32>) {
        let (rows, cols) = (pattern.len(), pattern[0].len());
        let (mut r, mut c) = (vec![true; rows - 1], vec![true; cols - 1]);
        #[allow(clippy::needless_range_loop)]
        for i in 0..rows {
            for j in 0..cols - 1 {
                if !c[j] {
                    continue;
                }
                let mut k = 0;
                while k <= j && j + k + 1 < cols {
                    if pattern[i][j - k] != pattern[i][j + k + 1] {
                        c[j] = false;
                        break;
                    }
                    k += 1;
                }
            }
        }
        for j in 0..cols {
            for i in 0..rows - 1 {
                if !r[i] {
                    continue;
                }
                let mut k = 0;
                while k <= i && i + k + 1 < rows {
                    if pattern[i - k][j] != pattern[i + k + 1][j] {
                        r[i] = false;
                        break;
                    }
                    k += 1;
                }
            }
        }
        (
            (1..)
                .zip(&c)
                .filter_map(|(i, b)| if *b { Some(i) } else { None })
                .collect(),
            (1..)
                .zip(&r)
                .filter_map(|(i, b)| if *b { Some(i * 100) } else { None })
                .collect(),
        )
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
            .map(|pattern| {
                let (cs, rs) = Self::summarized_numbers(pattern);
                assert!(cs.len() + rs.len() == 1);
                cs.first()
                    .copied()
                    .or(rs.first().copied())
                    .expect("should have a summarized number")
            })
            .sum()
    }
    fn part2(&self) -> Self::Answer2 {
        self.patterns
            .iter()
            .map(|pattern| {
                let (cs, rs) = Self::summarized_numbers(pattern);
                assert!(cs.len() + rs.len() == 1);
                let (rows, cols) = (pattern.len(), pattern[0].len());
                (0..rows)
                    .cartesian_product(0..cols)
                    .find_map(|(i, j)| {
                        let mut cloned = pattern.clone();
                        cloned[i][j] = !cloned[i][j];
                        let (new_cs, new_rs) = Self::summarized_numbers(&cloned);
                        new_cs
                            .iter()
                            .find(|n| !cs.contains(n))
                            .copied()
                            .or(new_rs.iter().find(|n| !rs.contains(n)).copied())
                    })
                    .expect("should have a new summarized number")
            })
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
