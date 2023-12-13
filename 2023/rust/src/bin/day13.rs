use aoc2023::Solve;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    patterns: Vec<Vec<Vec<char>>>,
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
                .map(|lines| lines.iter().map(|line| line.chars().collect()).collect())
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let mut ret = 0;
        for pattern in &self.patterns {
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
            let cs = (1..)
                .zip(&c)
                .filter_map(|(i, b)| if *b { Some(i) } else { None })
                .collect::<Vec<_>>();
            let rs = (1..)
                .zip(&r)
                .filter_map(|(i, b)| if *b { Some(i) } else { None })
                .collect::<Vec<_>>();
            assert!(cs.len() + rs.len() == 1);
            if !cs.is_empty() {
                ret += cs[0];
            }
            if !rs.is_empty() {
                ret += rs[0] * 100;
            }
        }
        ret
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
}
