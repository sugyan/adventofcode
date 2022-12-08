use aoc2021::Solve;
use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    energy_levels: Vec<Vec<u8>>,
}

impl Solution {
    fn count_flashes(grid: &mut [Vec<u8>]) -> u32 {
        let mut vd = VecDeque::new();
        (0..10).for_each(|i| (0..10).for_each(|j| vd.push_back((i, j))));
        while let Some((i, j)) = vd.pop_front() {
            grid[i][j] += 1;
            if grid[i][j] == 10 {
                [!0, 0, 1].iter().for_each(|&di| {
                    [!0, 0, 1].iter().for_each(|&dj| {
                        let i = i.wrapping_add(di);
                        let j = j.wrapping_add(dj);
                        if (0..10).contains(&i) && (0..10).contains(&j) {
                            vd.push_back((i, j));
                        }
                    });
                });
            }
        }
        let mut flashes = 0;
        grid.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|col| {
                if *col > 9 {
                    flashes += 1;
                    *col = 0;
                }
            });
        });
        flashes
    }
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        Self {
            energy_levels: BufReader::new(r)
                .lines()
                .filter_map(Result::ok)
                .map(|row| row.bytes().map(|u| u - b'0').collect())
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let mut grid = self.energy_levels.clone();
        (0..100).map(|_| Self::count_flashes(&mut grid)).sum()
    }
    fn part2(&self) -> Self::Answer2 {
        let mut grid = self.energy_levels.clone();
        for i in 1.. {
            if Self::count_flashes(&mut grid) == 100 {
                return i;
            }
        }
        unreachable!()
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
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(1656, Solution::new(example_input()).part1());
    }

    #[test]
    fn part2() {
        assert_eq!(195, Solution::new(example_input()).part2());
    }
}
