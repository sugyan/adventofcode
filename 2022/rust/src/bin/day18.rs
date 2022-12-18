use aoc2022::Solve;
use itertools::Itertools;
use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    cubes: Vec<(u32, u32, u32)>,
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;

    fn new(r: impl Read) -> Self {
        Self {
            cubes: BufReader::new(r)
                .lines()
                .filter_map(Result::ok)
                .filter_map(|line| {
                    line.split(',')
                        .filter_map(|s| s.parse().ok())
                        .collect_tuple()
                })
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let hs = self.cubes.iter().collect::<HashSet<_>>();
        hs.len() * 6
            - self
                .cubes
                .iter()
                .map(|&(x, y, z)| {
                    [(x + 1, y, z), (x, y + 1, z), (x, y, z + 1)]
                        .iter()
                        .filter(|p| hs.contains(p))
                        .count()
                        * 2
                })
                .sum::<usize>()
    }
    fn part2(&self) -> Self::Answer2 {
        todo!()
    }
}

fn main() {
    let solution = Solution::new(std::io::stdin().lock());
    println!("Part 1: {}", solution.part1());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static [u8] {
        r"
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(64, Solution::new(example_input()).part1());
    }
}
