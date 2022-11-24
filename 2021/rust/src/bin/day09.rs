use aoc2021::Solve;
use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    heightmap: Vec<Vec<u8>>,
    low_points: Vec<(usize, usize)>,
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        let heightmap = BufReader::new(r)
            .lines()
            .filter_map(Result::ok)
            .map(|line| line.bytes().map(|u| u - b'0').collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let (r, c) = (heightmap.len(), heightmap[0].len());
        let mut low_points = Vec::new();
        for (i, row) in heightmap.iter().enumerate() {
            for (j, &h) in row.iter().enumerate() {
                if [0, 1, 0, !0, 0].windows(2).all(|d| {
                    let i = i.wrapping_add(d[0]);
                    let j = j.wrapping_add(d[1]);
                    !(0..r).contains(&i) || !(0..c).contains(&j) || heightmap[i][j] > h
                }) {
                    low_points.push((i, j));
                }
            }
        }
        Self {
            heightmap,
            low_points,
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.low_points
            .iter()
            .map(|&(i, j)| self.heightmap[i][j] as u32 + 1)
            .sum()
    }
    fn part2(&self) -> Self::Answer2 {
        let (r, c) = (self.heightmap.len(), self.heightmap[0].len());
        let mut seen = vec![vec![false; self.heightmap[0].len()]; self.heightmap.len()];
        let mut sizes = self
            .low_points
            .iter()
            .map(|&(i, j)| {
                let mut queue = VecDeque::new();
                let mut size = 0;
                queue.push_back((i, j));
                while let Some((i, j)) = queue.pop_front() {
                    for d in [0, 1, 0, !0, 0].windows(2) {
                        let i = i.wrapping_add(d[0]);
                        let j = j.wrapping_add(d[1]);
                        if (0..r).contains(&i)
                            && (0..c).contains(&j)
                            && self.heightmap[i][j] < 9
                            && !seen[i][j]
                        {
                            size += 1;
                            seen[i][j] = true;
                            queue.push_back((i, j));
                        }
                    }
                }
                size
            })
            .collect::<Vec<_>>();
        sizes.sort_unstable();
        sizes.iter().rev().take(3).product()
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
2199943210
3987894921
9856789892
8767896789
9899965678"[1..]
            .as_bytes()
    }

    #[test]
    fn example1() {
        assert_eq!(15, Solution::new(example_input()).part1());
    }

    #[test]
    fn example2() {
        assert_eq!(1134, Solution::new(example_input()).part2());
    }
}
