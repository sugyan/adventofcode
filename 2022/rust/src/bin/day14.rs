use aoc2022::Solve;
use itertools::Itertools;
use std::io::{BufRead, BufReader};

struct Solution {
    cave: Vec<Vec<bool>>,
    ymax: usize,
}

impl Solution {
    fn count_units(&self, floor: bool) -> u32 {
        let mut block = self.cave.clone();
        let mut stack = vec![(500, 0)];
        let mut units = 0;
        while let Some(&(x, y)) = stack.last() {
            if let Some(&next) = [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)]
                .iter()
                .find(|p| !block[p.0][p.1])
            {
                if y < self.ymax + 1 {
                    stack.push(next);
                    continue;
                } else if !floor {
                    break;
                }
            }
            if let Some((x, y)) = stack.pop() {
                block[x][y] = true;
                units += 1;
            }
        }
        units
    }
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl std::io::Read) -> Self {
        let mut cave = vec![vec![false; 500]; 1001];
        let mut ymax = 0;
        for line in BufReader::new(r).lines().filter_map(Result::ok) {
            for (p0, p1) in line
                .split(" -> ")
                .filter_map(|xy| {
                    xy.split(',')
                        .filter_map(|s| s.parse().ok())
                        .collect_tuple::<(usize, usize)>()
                })
                .tuple_windows()
            {
                (p0.0.min(p1.0)..=p0.0.max(p1.0)).for_each(|x| cave[x][p0.1] = true);
                (p0.1.min(p1.1)..=p0.1.max(p1.1)).for_each(|y| cave[p0.0][y] = true);
                ymax = ymax.max(p0.1.max(p1.1));
            }
        }
        Self { cave, ymax }
    }
    fn part1(&self) -> Self::Answer1 {
        self.count_units(false)
    }
    fn part2(&self) -> Self::Answer2 {
        self.count_units(true)
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
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(24, Solution::new(example_input()).part1());
    }

    #[test]
    fn part2() {
        assert_eq!(93, Solution::new(example_input()).part2());
    }
}
