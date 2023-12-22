use aoc2023::Solve;
use itertools::Itertools;
use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    map: Vec<Vec<char>>,
    start: (usize, usize),
}

impl Solution {
    fn reach_count(&self, steps: usize) -> usize {
        let (rows, cols) = (self.map.len(), self.map[0].len());
        let mut mins = vec![vec![None; cols]; rows];
        mins[self.start.0][self.start.1] = Some(0);
        let mut vd = VecDeque::from([(self.start, 0)]);
        while let Some((p, d)) = vd.pop_front() {
            if d == steps {
                break;
            }
            let (i, j) = p;
            for (di, dj) in [(!0, 0), (1, 0), (0, !0), (0, 1)] {
                let (i, j) = (i.wrapping_add(di), j.wrapping_add(dj));
                if (0..rows).contains(&i)
                    && (0..cols).contains(&j)
                    && self.map[i][j] != '#'
                    && mins[i][j].is_none()
                {
                    mins[i][j] = Some(d + 1);
                    vd.push_back(((i, j), d + 1));
                }
            }
        }
        mins.iter()
            .flatten()
            .filter(|o| o.map_or(false, |x| x % 2 == 0))
            .count()
    }
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;

    fn new(r: impl Read) -> Self {
        let map = BufReader::new(r)
            .lines()
            .map_while(Result::ok)
            .map(|s| s.chars().collect_vec())
            .collect_vec();
        let start = (0..map.len())
            .cartesian_product(0..map[0].len())
            .find(|&(i, j)| map[i][j] == 'S')
            .expect("should have a start point");
        Self { map, start }
    }
    fn part1(&self) -> Self::Answer1 {
        self.reach_count(64)
    }
    fn part2(&self) -> Self::Answer2 {
        todo!()
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
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(Solution::new(example_input()).reach_count(6), 16);
    }
}
