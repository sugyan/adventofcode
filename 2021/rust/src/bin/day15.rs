use std::cmp::Reverse;
use std::collections::BinaryHeap;
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
        self.lowest_total(1)
    }
    fn part_2(&self) -> u32 {
        self.lowest_total(5)
    }
    fn lowest_total(&self, size: usize) -> u32 {
        let len = self.risk_levels.len();
        let grid = (0..len * size)
            .map(|i| {
                (0..len * size)
                    .map(|j| {
                        let offset = (i / len + j / len) as u32;
                        (self.risk_levels[i % len][j % len] as u32 + offset - 1) % 9 + 1
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut seen = vec![vec![false; len * size]; len * size];
        seen[0][0] = true;
        let mut bh = BinaryHeap::new();
        bh.push((Reverse(0), (0, 0)));
        while let Some((Reverse(total), (i, j))) = bh.pop() {
            if i == size * len - 1 && j == size * len - 1 {
                return total;
            }
            for d in [0, 1, 0, !0, 0].windows(2) {
                let i = i.wrapping_add(d[0]);
                let j = j.wrapping_add(d[1]);
                if (0..len * size).contains(&i) && (0..len * size).contains(&j) && !seen[i][j] {
                    seen[i][j] = true;
                    bh.push((Reverse(total + grid[i][j] as u32), (i, j)));
                }
            }
        }
        unreachable!()
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
    println!("{}", solution.part_2());
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

    #[test]
    fn example_2() {
        assert_eq!(315, Solution::new(&example_inputs()).part_2());
    }
}
