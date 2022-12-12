use aoc2022::Solve;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    heightmap: Vec<Vec<u8>>,
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        Self {
            heightmap: BufReader::new(r)
                .lines()
                .filter_map(Result::ok)
                .map(|s| s.bytes().collect())
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let (rows, cols) = (self.heightmap.len(), self.heightmap[0].len());
        let mut heightmap = self.heightmap.clone();
        let mut mins = vec![vec![None; cols]; rows];
        let mut bh = BinaryHeap::new();
        let mut e = (0, 0);
        for (i, row) in heightmap.iter_mut().enumerate() {
            for (j, h) in row.iter_mut().enumerate() {
                if *h == b'S' {
                    *h = b'a';
                    mins[i][j] = Some(0);
                    bh.push((Reverse(0), (i, j)));
                }
                if *h == b'E' {
                    *h = b'z';
                    e = (i, j);
                }
            }
        }
        while let Some((Reverse(steps), (i, j))) = bh.pop() {
            for d in [0, 1, 0, !0, 0].windows(2) {
                let ii = i.wrapping_add(d[0]);
                let jj = j.wrapping_add(d[1]);
                if (0..rows).contains(&ii)
                    && (0..cols).contains(&jj)
                    && heightmap[ii][jj] <= heightmap[i][j] + 1
                    && mins[ii][jj].is_none()
                {
                    mins[ii][jj] = Some(steps + 1);
                    bh.push((Reverse(steps + 1), (ii, jj)));
                }
            }
        }
        mins[e.0][e.1].unwrap()
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
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"[1..]
            .as_bytes()
    }

    #[test]
    fn test_part1() {
        assert_eq!(31, Solution::new(example_input()).part1());
    }
}
