use std::io::{BufRead, BufReader};

struct Solution {
    grid: Vec<Vec<bool>>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        Self {
            grid: inputs
                .iter()
                .map(|row| row.chars().map(|c| c == '#').collect())
                .collect(),
        }
    }
    fn part_1(&self) -> usize {
        self.count(3, 1)
    }
    fn part_2(&self) -> usize {
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
    let solution = Solution::new(
        &BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(Result::ok)
            .collect::<Vec<_>>(),
    );
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
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
        assert_eq!(7, Solution::new(&example_inputs()).part_1());
    }

    #[test]
    fn example_2() {
        assert_eq!(336, Solution::new(&example_inputs()).part_2());
    }
}
