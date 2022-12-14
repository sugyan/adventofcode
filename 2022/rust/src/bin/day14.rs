use aoc2022::Solve;
use itertools::Itertools;
use std::collections::HashSet;
use std::io::{BufRead, BufReader};

struct Solution {
    rocks: HashSet<(i32, i32)>,
    ymax: i32,
}

impl Solution {
    fn count_units(&self, floor: bool) -> u32 {
        let mut hs = self.rocks.clone();
        for u in 0.. {
            let mut sand = (500, 0);
            while sand.1 < self.ymax + 1 {
                if let Some(x) = [0, -1, 1]
                    .iter()
                    .find(|&dx| !hs.contains(&(sand.0 + dx, sand.1 + 1)))
                {
                    sand = (sand.0 + x, sand.1 + 1);
                } else {
                    break;
                }
            }
            if (!floor && sand.1 > self.ymax) || hs.contains(&(500, 0)) {
                return u;
            }
            hs.insert(sand);
        }
        unreachable!();
    }
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl std::io::Read) -> Self {
        let mut rocks = HashSet::new();
        for line in BufReader::new(r).lines().filter_map(Result::ok) {
            for (p0, p1) in line
                .split(" -> ")
                .filter_map(|xy| {
                    xy.split(',')
                        .filter_map(|s| s.parse().ok())
                        .collect_tuple::<(i32, i32)>()
                })
                .tuple_windows()
            {
                rocks.extend((p0.0.min(p1.0)..=p0.0.max(p1.0)).map(|x| (x, p0.1)));
                rocks.extend((p0.1.min(p1.1)..=p0.1.max(p1.1)).map(|y| (p0.0, y)));
            }
        }
        let ymax = *rocks.iter().map(|(_, y)| y).max().unwrap();
        Self { rocks, ymax }
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
