use aoc2022::Solve;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    grid: Vec<Vec<u8>>,
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;

    fn new(r: impl Read) -> Self {
        Self {
            grid: BufReader::new(r)
                .lines()
                .filter_map(Result::ok)
                .map(|line| line.bytes().map(|b| b - b'0').collect())
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let (rows, cols) = (self.grid.len(), self.grid[0].len());
        let mut grid = vec![vec![false; cols]; rows];
        #[allow(clippy::needless_range_loop)]
        for i in 0..rows {
            (0..cols).fold(0, |max, j| {
                grid[i][j] |= j == 0 || self.grid[i][j] > max;
                max.max(self.grid[i][j])
            });
            (0..cols).rev().fold(0, |max, j| {
                grid[i][j] |= j == cols - 1 || self.grid[i][j] > max;
                max.max(self.grid[i][j])
            });
        }
        for j in 0..cols {
            (0..rows).fold(0, |max, i| {
                grid[i][j] |= i == 0 || self.grid[i][j] > max;
                max.max(self.grid[i][j])
            });
            (0..rows).rev().fold(0, |max, i| {
                grid[i][j] |= i == rows - 1 || self.grid[i][j] > max;
                max.max(self.grid[i][j])
            });
        }
        grid.iter()
            .map(|row| row.iter().filter(|&b| *b).count())
            .sum()
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
}
