use aoc2022::Solve;
use itertools::Itertools;
use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    min_steps: Vec<Vec<Option<(u32, char)>>>,
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        let heightmap = BufReader::new(r)
            .lines()
            .filter_map(Result::ok)
            .map(|s| s.bytes().collect_vec())
            .collect_vec();
        let (rows, cols) = (heightmap.len(), heightmap[0].len());
        let mut vd = VecDeque::new();
        for (i, row) in heightmap.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if *col == b'E' {
                    vd.push_back(((i, j), 0));
                }
            }
        }
        let mut min_steps = vec![vec![None; cols]; rows];
        while let Some(((i, j), steps)) = vd.pop_front() {
            if min_steps[i][j].is_some() {
                continue;
            }
            min_steps[i][j] = Some((steps, heightmap[i][j] as char));
            let height = match heightmap[i][j] {
                b'E' => b'z',
                h => h,
            };
            for d in [0, 1, 0, !0, 0].windows(2) {
                let ii = i.wrapping_add(d[0]);
                let jj = j.wrapping_add(d[1]);
                if (0..rows).contains(&ii)
                    && (0..cols).contains(&jj)
                    && (heightmap[ii][jj] >= height - 1
                        || (heightmap[ii][jj] == b'S' && height <= b'b'))
                {
                    vd.push_back(((ii, jj), steps + 1));
                }
            }
        }
        Self { min_steps }
    }
    fn part1(&self) -> Self::Answer1 {
        self.min_steps
            .iter()
            .flatten()
            .find_map(|o| match o {
                Some((steps, 'S')) => Some(*steps),
                _ => None,
            })
            .unwrap()
    }
    fn part2(&self) -> Self::Answer2 {
        self.min_steps
            .iter()
            .flatten()
            .filter_map(|o| match o {
                Some((steps, 'a' | 'S')) => Some(*steps),
                _ => None,
            })
            .min()
            .unwrap()
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
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(31, Solution::new(example_input()).part1());
    }

    #[test]
    fn part2() {
        assert_eq!(29, Solution::new(example_input()).part2());
    }
}
