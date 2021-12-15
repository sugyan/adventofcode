use std::io::{BufRead, BufReader};

struct Solution {
    risk_levels: Vec<Vec<u8>>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        Self {
            risk_levels: inputs
                .iter()
                .map(|s| s.bytes().map(|u| u - b'0').collect())
                .collect(),
        }
    }
    fn part_1(&self) -> u32 {
        let (rows, cols) = (self.risk_levels.len(), self.risk_levels[0].len());
        let mut grid = vec![vec![0; cols]; rows];
        for i in 0..rows {
            for j in 0..cols {
                grid[i][j] = self.risk_levels[i][j] as u32
                    + match (i > 0, j > 0) {
                        (false, false) => 0,
                        (false, true) => grid[i][j - 1],
                        (true, false) => grid[i - 1][j],
                        (true, true) => grid[i][j - 1].min(grid[i - 1][j]),
                    }
            }
        }
        grid[rows - 1][cols - 1] - grid[0][0]
    }
}

fn main() {
    let solution = Solution::new(
        &BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(Result::ok)
            .collect::<Vec<_>>(),
    );
    println!("{}", solution.part_1());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_inputs() -> Vec<String> {
        r"
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"[1..]
            .split('\n')
            .map(String::from)
            .collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(40, Solution::new(&example_inputs()).part_1());
    }
}
