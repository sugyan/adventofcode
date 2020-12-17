use std::io::{BufRead, BufReader};

struct Solution {
    grid: Vec<Vec<Vec<bool>>>,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        let (x, y) = (inputs[0].len(), inputs.len());
        let mut grid = vec![vec![vec![false; x + 12]; y + 12]; 13];
        for (i, row) in inputs.iter().enumerate() {
            for (j, c) in row.chars().enumerate() {
                grid[6][i + 6][j + 6] = c == '#'
            }
        }
        Self { grid }
    }
    fn solve_1(&self) -> usize {
        let mut grid = self.grid.clone();
        let mut d: Vec<(i32, i32, i32)> = Vec::with_capacity(26);
        for i in -1..=1 {
            for j in -1..=1 {
                for k in -1..=1 {
                    if i == 0 && j == 0 && k == 0 {
                        continue;
                    }
                    d.push((i, j, k));
                }
            }
        }
        for _ in 0..6 {
            grid = grid
                .iter()
                .enumerate()
                .map(|(i, plane)| {
                    plane
                        .iter()
                        .enumerate()
                        .map(|(j, row)| {
                            row.iter()
                                .enumerate()
                                .map(|(k, &b)| {
                                    let neighbors = d
                                        .iter()
                                        .filter(|&d| {
                                            let z = i as i32 + d.0;
                                            let y = j as i32 + d.1;
                                            let x = k as i32 + d.2;
                                            z >= 0
                                                && y >= 0
                                                && x >= 0
                                                && z < grid.len() as i32
                                                && y < grid[0].len() as i32
                                                && x < grid[0][0].len() as i32
                                                && grid[z as usize][y as usize][x as usize]
                                        })
                                        .count();
                                    match b {
                                        true if neighbors != 2 && neighbors != 3 => false,
                                        false if neighbors == 3 => true,
                                        b => b,
                                    }
                                })
                                .collect()
                        })
                        .collect()
                })
                .collect();
        }
        grid.iter()
            .map(|plane| {
                plane
                    .iter()
                    .map(|row| row.iter().filter(|&&b| b).count())
                    .sum::<usize>()
            })
            .sum()
    }
}

fn main() {
    let solution = Solution::new(
        BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(|line| line.ok())
            .collect(),
    );
    println!("{}", solution.solve_1());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            112,
            Solution::new(
                "
.#.
..#
###"[1..]
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect()
            )
            .solve_1()
        );
    }
}
