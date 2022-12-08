use aoc2022::Solve;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    grid: Vec<Vec<u8>>,
    distances: Vec<Vec<[usize; 4]>>,
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
        let distance = |(i, j): (usize, usize), d: (usize, usize)| {
            let (mut ii, mut jj) = (i, j);
            let mut ret = 0;
            while {
                (ii, jj) = (ii.wrapping_add(d.0), jj.wrapping_add(d.1));
                (0..rows).contains(&ii) && (0..cols).contains(&jj)
            } {
                ret += 1;
                if grid[ii][jj] >= grid[i][j] {
                    break;
                }
            }
            ret
        };
        let distances = (0..rows)
            .map(|i| {
                (0..cols)
                    .map(|j| [(!0, 0), (1, 0), (0, !0), (0, 1)].map(|d| distance((i, j), d)))
                    .collect()
            })
            .collect();
        Self { grid, distances }
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
        self.distances
            .iter()
            .flatten()
            .map(|d| d.iter().product())
            .max()
            .unwrap()
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
