use aoc2022::Solve;
use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read};

const NO_BLIZZARD: [bool; 4] = [false, false, false, false];

struct Solution {
    valley: Vec<Vec<[bool; 4]>>,
    rows: usize,
    cols: usize,
}

impl Solution {
    fn fewest_minutes(
        &self,
        valley: &mut Vec<Vec<[bool; 4]>>,
        start: (usize, usize),
        goal: (usize, usize),
    ) -> u32 {
        let mut hs = HashSet::<(usize, usize)>::new();
        for minutes in 1.. {
            *valley = (0..self.rows)
                .map(|i| {
                    (0..self.cols)
                        .map(|j| {
                            [
                                valley[(i + 1) % self.rows][j][0],
                                valley[(i + self.rows - 1) % self.rows][j][1],
                                valley[i][(j + 1) % self.cols][2],
                                valley[i][(j + self.cols - 1) % self.cols][3],
                            ]
                        })
                        .collect()
                })
                .collect();
            if hs.contains(&goal) {
                return minutes;
            }
            hs = hs
                .iter()
                .flat_map(|(i, j)| {
                    [(!0, 0), (0, !0), (0, 0), (0, 1), (1, 0)]
                        .iter()
                        .filter_map(|&(di, dj)| {
                            Some((i.wrapping_add(di), j.wrapping_add(dj))).filter(|(i, j)| {
                                (0..self.rows).contains(i) && (0..self.cols).contains(j)
                            })
                        })
                        .filter(|&(i, j)| valley[i][j] == NO_BLIZZARD)
                        .collect::<Vec<_>>()
                })
                .collect();
            if valley[start.0][start.1] == NO_BLIZZARD {
                hs.insert(start);
            }
        }
        unreachable!();
    }
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        let lines = BufReader::new(r)
            .lines()
            .filter_map(Result::ok)
            .collect::<Vec<_>>();
        Self {
            valley: lines[1..lines.len() - 1]
                .iter()
                .map(|s| {
                    s[1..s.len() - 1]
                        .chars()
                        .map(|c| match c {
                            '.' => NO_BLIZZARD,
                            '^' => [true, false, false, false],
                            'v' => [false, true, false, false],
                            '<' => [false, false, true, false],
                            '>' => [false, false, false, true],
                            _ => unreachable!(),
                        })
                        .collect()
                })
                .collect(),
            rows: lines.len() - 2,
            cols: lines[1].len() - 2,
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.fewest_minutes(
            &mut self.valley.clone(),
            (0, 0),
            (self.rows - 1, self.cols - 1),
        )
    }
    fn part2(&self) -> Self::Answer2 {
        let mut valley = self.valley.clone();
        [
            self.fewest_minutes(&mut valley, (0, 0), (self.rows - 1, self.cols - 1)),
            self.fewest_minutes(&mut valley, (self.rows - 1, self.cols - 1), (0, 0)),
            self.fewest_minutes(&mut valley, (0, 0), (self.rows - 1, self.cols - 1)),
        ]
        .iter()
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
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(18, Solution::new(example_input()).part1());
    }

    #[test]
    fn part2() {
        assert_eq!(54, Solution::new(example_input()).part2());
    }
}
