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
        self.count(3, 1)
    }
    fn solve_2(&self) -> usize {
        [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(|&(right, down)| self.count(right, down))
            .product()
    }
    fn count(&self, right: usize, down: usize) -> usize {
        let width = self.grid[0].len();
        (0..self.grid.len() / down)
            .filter(|&i| self.grid[i * down][(i * right) % width])
            .count()
    }
}

fn main() {
    let inputs: Vec<String> = BufReader::new(std::io::stdin().lock())
        .lines()
        .filter_map(|line| line.ok())
        .collect();
    let solution = Solution::new(inputs);
    println!("{}", solution.solve_1());
    println!("{}", solution.solve_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_inputs() -> Vec<String> {
        r"
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
.#..#...#.#"
            .split('\n')
            .skip(1)
            .map(str::to_string)
            .collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(7, Solution::new(example_inputs()).solve_1());
    }

    #[test]
    fn example_2() {
        assert_eq!(336, Solution::new(example_inputs()).solve_2());
    }
}
