use aoc2022::Solve;
use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read};

const NO_BLIZZARD: [bool; 4] = [false, false, false, false];

struct Solution {
    valley: Vec<Vec<[bool; 4]>>,
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
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let mut valley = self.valley.clone();
        let (rows, cols) = (valley.len(), valley[0].len());
        let mut hs = HashSet::<(usize, usize)>::new();
        for minutes in 1.. {
            valley = (0..rows)
                .map(|i| {
                    (0..cols)
                        .map(|j| {
                            [
                                valley[(i + 1) % rows][j][0],
                                valley[(i + rows - 1) % rows][j][1],
                                valley[i][(j + 1) % cols][2],
                                valley[i][(j + cols - 1) % cols][3],
                            ]
                        })
                        .collect()
                })
                .collect();
            hs = hs
                .iter()
                .flat_map(|(i, j)| {
                    [(!0, 0), (0, !0), (0, 0), (0, 1), (1, 0)]
                        .iter()
                        .filter_map(|&(di, dj)| {
                            Some((i.wrapping_add(di), j.wrapping_add(dj)))
                                .filter(|(i, j)| (0..rows).contains(i) && (0..cols).contains(j))
                        })
                        .filter(|&(i, j)| valley[i][j] == NO_BLIZZARD)
                        .collect::<Vec<_>>()
                })
                .collect();
            if valley[0][0] == NO_BLIZZARD {
                hs.insert((0, 0));
            }
            if hs.contains(&(rows - 1, cols - 1)) {
                return minutes + 1;
            }
        }
        unreachable!();
    }
    fn part2(&self) -> Self::Answer2 {
        todo!()
    }
}

fn main() {
    let solution = Solution::new(std::io::stdin().lock());
    println!("Part 1: {}", solution.part1());
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
}
