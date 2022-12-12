use aoc2022::Solve;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    start: (usize, usize),
    min_steps: Vec<Vec<Option<(u32, u8)>>>,
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        let mut heightmap = BufReader::new(r)
            .lines()
            .filter_map(Result::ok)
            .map(|s| s.bytes().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let (rows, cols) = (heightmap.len(), heightmap[0].len());
        let mut min_steps = vec![vec![None; cols]; rows];
        let mut bh = BinaryHeap::new();
        let mut start = (0, 0);
        for (i, row) in heightmap.iter_mut().enumerate() {
            for (j, h) in row.iter_mut().enumerate() {
                match *h {
                    b'E' => {
                        *h = b'z';
                        min_steps[i][j] = Some((0, b'z'));
                        bh.push((Reverse(0), (i, j)));
                    }
                    b'S' => {
                        *h = b'a';
                        start = (i, j);
                    }
                    _ => {}
                }
            }
        }
        while let Some((Reverse(steps), (i, j))) = bh.pop() {
            for d in [0, 1, 0, !0, 0].windows(2) {
                let ii = i.wrapping_add(d[0]);
                let jj = j.wrapping_add(d[1]);
                if (0..rows).contains(&ii)
                    && (0..cols).contains(&jj)
                    && heightmap[ii][jj] + 1 >= heightmap[i][j]
                    && min_steps[ii][jj].is_none()
                {
                    min_steps[ii][jj] = Some((steps + 1, heightmap[ii][jj]));
                    bh.push((Reverse(steps + 1), (ii, jj)));
                }
            }
        }
        Self { start, min_steps }
    }
    fn part1(&self) -> Self::Answer1 {
        self.min_steps[self.start.0][self.start.1].unwrap().0
    }
    fn part2(&self) -> Self::Answer2 {
        self.min_steps
            .iter()
            .flatten()
            .filter_map(|x| match x {
                Some((steps, h)) if *h == b'a' => Some(*steps),
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
    fn test_part1() {
        assert_eq!(31, Solution::new(example_input()).part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(29, Solution::new(example_input()).part2());
    }
}
