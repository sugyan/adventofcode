use aoc2021::Solve;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    risk_levels: Vec<Vec<u8>>,
}

impl Solution {
    fn dijkstra(&self, size: usize) -> u32 {
        let len = self.risk_levels.len();
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
                    let risk_level = ((i / len + j / len) as u32
                        + (self.risk_levels[i % len][j % len] as u32)
                        - 1)
                        % 9
                        + 1;
                    bh.push((Reverse(total + risk_level), (i, j)));
                }
            }
        }
        unreachable!()
    }
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        Self {
            risk_levels: BufReader::new(r)
                .lines()
                .filter_map(Result::ok)
                .map(|s| s.bytes().map(|u| u - b'0').collect())
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.dijkstra(1)
    }
    fn part2(&self) -> Self::Answer2 {
        self.dijkstra(5)
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
            .as_bytes()
    }

    #[test]
    fn example1() {
        assert_eq!(40, Solution::new(example_input()).part1());
    }

    #[test]
    fn example2() {
        assert_eq!(315, Solution::new(example_input()).part2());
    }
}
