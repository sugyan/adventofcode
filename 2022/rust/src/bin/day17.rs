use aoc2022::Solve;
use itertools::Itertools;
use std::collections::HashMap;
use std::io::Read;

const ROCKS: [[u8; 4]; 5] = [
    [0b0001_1110, 0b0000_0000, 0b0000_0000, 0b0000_0000],
    [0b0000_1000, 0b0001_1100, 0b0000_1000, 0b0000_0000],
    [0b0001_1100, 0b0000_0100, 0b0000_0100, 0b0000_0000],
    [0b0001_0000, 0b0001_0000, 0b0001_0000, 0b0001_0000],
    [0b0001_1000, 0b0001_1000, 0b0000_0000, 0b0000_0000],
];

enum Direction {
    Left,
    Right,
}

struct Solution {
    jet_patterns: Vec<Direction>,
}

impl Solution {
    fn tower_height(&self, num_rocks: usize) -> u64 {
        let mut tower = vec![0; 0];
        let mut hm = HashMap::new();
        let mut ij = 0;
        for i in 0..num_rocks {
            let ir = i % ROCKS.len();
            let key = (ir, ij, tower.iter().rev().take(4).cloned().collect_vec());
            if let Some((pi, plen)) = hm.get(&key) {
                if (num_rocks - i) % (i - pi) == 0 {
                    return ((num_rocks - i) / (i - pi) * (tower.len() - plen) + tower.len())
                        as u64;
                }
            } else {
                hm.insert(key, (i, tower.len()));
            }
            let mut rock = ROCKS[ir];
            let mut j = tower.len() + 3;
            tower.extend(vec![0; 7]);
            loop {
                let jet = &self.jet_patterns[ij];
                ij = (ij + 1) % self.jet_patterns.len();
                match jet {
                    Direction::Left => {
                        if rock
                            .iter()
                            .enumerate()
                            .all(|(k, u)| u & 0x40 == 0 && tower[j + k] & (u << 1) == 0)
                        {
                            rock.iter_mut().for_each(|u| *u <<= 1);
                        }
                    }
                    Direction::Right => {
                        if rock
                            .iter()
                            .enumerate()
                            .all(|(k, u)| u & 0x01 == 0 && tower[j + k] & (u >> 1) == 0)
                        {
                            rock.iter_mut().for_each(|u| *u >>= 1);
                        }
                    }
                }
                if j == 0
                    || rock
                        .iter()
                        .enumerate()
                        .any(|(k, u)| u & tower[j + k - 1] != 0)
                {
                    break;
                }
                j -= 1;
            }
            rock.iter().enumerate().for_each(|(k, u)| tower[j + k] |= u);
            while let Some(&last) = tower.last() {
                if last == 0 {
                    tower.pop();
                } else {
                    break;
                }
            }
        }
        tower.len() as u64
    }
}

impl Solve for Solution {
    type Answer1 = u64;
    type Answer2 = u64;

    fn new(mut r: impl Read) -> Self {
        let mut buf = String::new();
        r.read_to_string(&mut buf).ok();
        Self {
            jet_patterns: buf
                .trim()
                .chars()
                .map(|c| match c {
                    '<' => Direction::Left,
                    '>' => Direction::Right,
                    _ => unreachable!(),
                })
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.tower_height(2022)
    }
    fn part2(&self) -> Self::Answer2 {
        self.tower_height(1_000_000_000_000)
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
>>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(3068, Solution::new(example_input()).part1());
    }

    #[test]
    fn part2() {
        assert_eq!(1_514_285_714_288, Solution::new(example_input()).part2());
    }
}
