use std::io::{BufRead, BufReader};

struct Solution {
    grid: Vec<Vec<bool>>,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        Self {
            grid: inputs
                .iter()
                .map(|row| row.chars().map(|c| c == '#').collect())
                .collect(),
        }
    }
    fn solve_1(&self) -> usize {
        let width = self.grid[0].len();
        (0..self.grid.len())
            .filter(|&i| self.grid[i][(i * 3) % width])
            .count()
    }
}

fn main() {
    let inputs: Vec<String> = BufReader::new(std::io::stdin().lock())
        .lines()
        .filter_map(|line| line.ok())
        .collect();
    println!("{}", Solution::new(inputs).solve_1());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            7,
            Solution::new(
                "
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"[1..]
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect()
            )
            .solve_1()
        );
    }
}
