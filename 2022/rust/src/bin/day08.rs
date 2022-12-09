use aoc2022::Solve;
use itertools::Itertools;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    trees: Vec<(bool, usize)>,
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;

    fn new(r: impl Read) -> Self {
        let grid = BufReader::new(r)
            .lines()
            .filter_map(Result::ok)
            .map(|line| line.bytes().map(|b| b - b'0').collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let (rows, cols) = (grid.len(), grid[0].len());
        Self {
            trees: (0..rows)
                .cartesian_product(0..cols)
                .map(|(i, j)| {
                    let h = grid[i][j];
                    [
                        (0..i).rev().map(|ii| grid[ii][j]).collect::<Vec<_>>(),
                        (0..j).rev().map(|jj| grid[i][jj]).collect::<Vec<_>>(),
                        (i + 1..rows).map(|ii| grid[ii][j]).collect::<Vec<_>>(),
                        (j + 1..cols).map(|jj| grid[i][jj]).collect::<Vec<_>>(),
                    ]
                    .iter()
                    .fold((false, 1), |acc, x| {
                        (
                            acc.0 | x.iter().all(|&e| e < h),
                            acc.1 * x.iter().position(|&e| e >= h).map_or(x.len(), |p| p + 1),
                        )
                    })
                })
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.trees.iter().filter(|(visible, _)| *visible).count()
    }
    fn part2(&self) -> Self::Answer2 {
        self.trees.iter().map(|(_, score)| *score).max().unwrap()
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
30373
25512
65332
33549
35390
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(21, Solution::new(example_input()).part1());
    }

    #[test]
    fn part2() {
        assert_eq!(8, Solution::new(example_input()).part2());
    }
}
