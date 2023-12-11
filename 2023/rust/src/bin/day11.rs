use aoc2023::Solve;
use itertools::Itertools;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    image: Vec<Vec<bool>>,
}

impl Solution {
    fn sum_of_length(&self, times: usize) -> usize {
        let (rows, cols) = (self.image.len(), self.image[0].len());
        let space_rows = (0..rows)
            .filter(|&i| (0..cols).all(|j| !self.image[i][j]))
            .collect_vec();
        let space_cols = (0..cols)
            .filter(|&j| (0..rows).all(|i| !self.image[i][j]))
            .collect_vec();
        let galaxies = (0..rows)
            .cartesian_product(0..cols)
            .filter(|&(i, j)| self.image[i][j])
            .collect_vec();
        galaxies
            .iter()
            .combinations(2)
            .map(|c| {
                let (rmin, rmax) = (c[0].0.min(c[1].0), c[0].0.max(c[1].0));
                let (cmin, cmax) = (c[0].1.min(c[1].1), c[0].1.max(c[1].1));
                let mut length = (rmax - rmin) + (cmax - cmin);
                if let (Err(min), Err(max)) = (
                    space_rows.binary_search(&rmin),
                    space_rows.binary_search(&rmax),
                ) {
                    length += (max - min) * (times - 1);
                }
                if let (Err(min), Err(max)) = (
                    space_cols.binary_search(&cmin),
                    space_cols.binary_search(&cmax),
                ) {
                    length += (max - min) * (times - 1);
                }
                length
            })
            .sum()
    }
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;

    fn new(r: impl Read) -> Self {
        Self {
            image: BufReader::new(r)
                .lines()
                .map_while(Result::ok)
                .map(|line| line.chars().map(|c| c == '#').collect())
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.sum_of_length(2)
    }
    fn part2(&self) -> Self::Answer2 {
        self.sum_of_length(1_000_000)
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
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(Solution::new(example_input()).part1(), 374);
    }

    #[test]
    fn sum_of_length() {
        assert_eq!(Solution::new(example_input()).sum_of_length(10), 1030);
        assert_eq!(Solution::new(example_input()).sum_of_length(100), 8410);
    }
}
