use aoc2023::Solve;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};

enum Tilt {
    North,
    West,
    South,
    East,
}

struct Solution {
    platform: Vec<Vec<char>>,
}

impl Solution {
    fn tilt(tilt: Tilt, platform: &mut [Vec<char>]) {
        let (rows, cols) = (platform.len(), platform[0].len());
        let (starts, (di, dj)) = match tilt {
            Tilt::North => ((0..cols).map(|j| (0, j)).collect::<Vec<_>>(), (1, 0)),
            Tilt::West => ((0..rows).map(|i| (i, 0)).collect::<Vec<_>>(), (0, 1)),
            Tilt::South => (
                (0..cols).map(|j| (rows - 1, j)).collect::<Vec<_>>(),
                (!0, 0),
            ),
            Tilt::East => (
                (0..rows).map(|i| (i, cols - 1)).collect::<Vec<_>>(),
                (0, !0),
            ),
        };
        for (mut i, mut j) in starts {
            let (mut ii, mut jj) = (i, j);
            while (0..rows).contains(&i) && (0..cols).contains(&j) {
                match platform[i][j] {
                    '#' => {
                        (ii, jj) = (i.wrapping_add(di), j.wrapping_add(dj));
                    }
                    'O' => {
                        if ii != i || jj != j {
                            platform[ii][jj] = 'O';
                            platform[i][j] = '.';
                        }
                        (ii, jj) = (ii.wrapping_add(di), jj.wrapping_add(dj));
                    }
                    _ => {}
                }
                (i, j) = (i.wrapping_add(di), j.wrapping_add(dj));
            }
        }
    }
    fn total_load(platform: &[Vec<char>]) -> usize {
        platform
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .filter_map(|c| {
                        if *c == 'O' {
                            Some(platform.len() - i)
                        } else {
                            None
                        }
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;

    fn new(r: impl Read) -> Self {
        Self {
            platform: BufReader::new(r)
                .lines()
                .map_while(Result::ok)
                .map(|line| line.chars().collect())
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let mut platform = self.platform.clone();
        Self::tilt(Tilt::North, &mut platform);
        Self::total_load(&platform)
    }
    fn part2(&self) -> Self::Answer2 {
        let mut platform = self.platform.clone();
        let mut hm = HashMap::new();
        for i in 1.. {
            Self::tilt(Tilt::North, &mut platform);
            Self::tilt(Tilt::West, &mut platform);
            Self::tilt(Tilt::South, &mut platform);
            Self::tilt(Tilt::East, &mut platform);
            if let Some((j, _)) = hm.get(&platform) {
                let k = (1_000_000_000 - i) % (i - j);
                return hm
                    .values()
                    .find_map(|&(i, load)| if j + k == i { Some(load) } else { None })
                    .expect("should have value");
            } else {
                hm.insert(platform.clone(), (i, Self::total_load(&platform)));
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
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(Solution::new(example_input()).part1(), 136);
    }

    #[test]
    fn part2() {
        assert_eq!(Solution::new(example_input()).part2(), 64);
    }
}
