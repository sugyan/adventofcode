use std::io::{BufRead, BufReader};

struct Solution {
    lines: Vec<((usize, usize), (usize, usize))>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        Self {
            lines: inputs
                .iter()
                .map(|line| {
                    let points = line
                        .split(" -> ")
                        .map(|s| {
                            s.split(',')
                                .map(|s| s.parse::<usize>().unwrap())
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>();
                    ((points[0][0], points[0][1]), (points[1][0], points[1][1]))
                })
                .collect(),
        }
    }
    fn part_1(&self) -> usize {
        let xmax = self
            .lines
            .iter()
            .map(|((x1, _), (x2, _))| x1.max(x2))
            .max()
            .unwrap();
        let ymax = self
            .lines
            .iter()
            .map(|((_, y1), (_, y2))| y1.max(y2))
            .max()
            .unwrap();
        let mut grid = vec![vec![0; xmax + 1]; ymax + 1];
        for &((x1, y1), (x2, y2)) in &self.lines {
            if x1 == x2 {
                for y in y1.min(y2)..=y1.max(y2) {
                    grid[y][x1] += 1;
                }
            }
            if y1 == y2 {
                for x in x1.min(x2)..=x1.max(x2) {
                    grid[y1][x] += 1;
                }
            }
        }
        grid.iter()
            .map(|row| row.iter().filter(|&&v| v > 1).count())
            .sum()
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
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"[1..]
            .split('\n')
            .map(String::from)
            .collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(5, Solution::new(&example_inputs()).part_1());
    }
}
