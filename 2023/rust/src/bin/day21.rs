use aoc2023::Solve;
use itertools::Itertools;
use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    map: Vec<Vec<char>>,
    start: (usize, usize),
}

impl Solution {
    fn reachable_count(&self, steps: usize) -> usize {
        if steps <= 65 {
            self.reachable_count_from(steps, self.start, 0)
        } else {
            // 26501365 = 202300 * 131 + 65
            let n = 202_300_usize;
            let (rows, cols) = (self.map.len(), self.map[0].len());
            [
                self.reachable_count_from(130, self.start, 1) * ((n - 1) / 2 * 2 + 1).pow(2),
                self.reachable_count_from(130, self.start, 0) * ((n / 2) * 2).pow(2),
                self.reachable_count_from(130, (rows / 2, 0), 0),
                self.reachable_count_from(130, (rows / 2, cols - 1), 0),
                self.reachable_count_from(130, (0, cols / 2), 0),
                self.reachable_count_from(130, (rows - 1, cols / 2), 0),
                self.reachable_count_from(64, (0, 0), 0) * n,
                self.reachable_count_from(64, (rows - 1, 0), 0) * n,
                self.reachable_count_from(64, (0, cols - 1), 0) * n,
                self.reachable_count_from(64, (rows - 1, cols - 1), 0) * n,
                self.reachable_count_from(64 + 131, (0, 0), 1) * (n - 1),
                self.reachable_count_from(64 + 131, (rows - 1, 0), 1) * (n - 1),
                self.reachable_count_from(64 + 131, (0, cols - 1), 1) * (n - 1),
                self.reachable_count_from(64 + 131, (rows - 1, cols - 1), 1) * (n - 1),
            ]
            .iter()
            .sum()
        }
    }
    fn reachable_count_from(&self, steps: usize, start: (usize, usize), target: usize) -> usize {
        let (rows, cols) = (self.map.len(), self.map[0].len());
        let mut mins = vec![vec![None; cols]; rows];
        mins[start.0][start.1] = Some(0);
        let mut vd = VecDeque::from([(start, 0)]);
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
            .filter(|o| o.map_or(false, |x| x % 2 == target))
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
        self.reachable_count(64)
    }
    fn part2(&self) -> Self::Answer2 {
        self.reachable_count(26_501_365)
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
        assert_eq!(Solution::new(example_input()).reachable_count(6), 16);
    }

    // #[test]
    // fn part2() {
    //     assert_eq!(Solution::new(example_input()).reach_count(10), 50);
    // }
}
