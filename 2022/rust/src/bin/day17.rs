use aoc2022::Solve;
use std::collections::HashMap;
use std::io::{BufReader, Read};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

struct Solution {
    jet_patterns: Vec<Direction>,
}

impl Solution {
    fn tower_height(&self, target: u64) -> u64 {
        let mut rocks = [
            vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
            vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
            vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        ]
        .into_iter()
        .cycle();
        let mut chamber = vec![[false; 7]; 0];
        let (mut i, mut j) = (0, 0);
        let mut hm = HashMap::new();
        while i < target {
            let mut rock = rocks
                .next()
                .unwrap()
                .iter()
                .map(|(i, j)| (i + chamber.len() + 3, j + 2))
                .collect::<Vec<_>>();
            chamber.extend(vec![[false; 7]; 7]);
            loop {
                match self.jet_patterns[j % self.jet_patterns.len()] {
                    Direction::Left => {
                        if rock.iter().all(|&(i, j)| j > 0 && !chamber[i][j - 1]) {
                            rock.iter_mut().for_each(|(_, j)| *j -= 1);
                        }
                    }
                    Direction::Right => {
                        if rock.iter().all(|&(i, j)| j < 6 && !chamber[i][j + 1]) {
                            rock.iter_mut().for_each(|(_, j)| *j += 1);
                        }
                    }
                }
                j += 1;
                if rock.iter().any(|&(i, j)| i == 0 || chamber[i - 1][j]) {
                    break;
                }
                rock.iter_mut().for_each(|(i, _)| *i -= 1);
            }
            rock.iter().for_each(|&(i, j)| chamber[i][j] = true);
            while let Some(last) = chamber.last() {
                if last.iter().all(|&x| !x) {
                    chamber.pop();
                } else {
                    break;
                }
            }
            let key = (i % 5, j % self.jet_patterns.len(), *chamber.last().unwrap());
            hm.entry(key)
                .or_insert_with(Vec::new)
                .push((i, chamber.len() as u64));
            if hm[&key].len() > 2 {
                let mut v = hm[&key]
                    .windows(2)
                    .map(|w| (w[1].0 - w[0].0, w[1].1 - w[0].1))
                    .collect::<Vec<_>>();
                v.dedup();
                if v.len() == 1 {
                    let (cycle, offset) = v[0];
                    let remaining = target - i;
                    if let Some((_, h)) = hm
                        .values()
                        .filter_map(|v| v.last())
                        .find(|(prev, _)| *prev == i - cycle + remaining % cycle - 1)
                    {
                        return (remaining / cycle + 1) * offset + h;
                    }
                }
            }
            i += 1;
        }
        chamber.len() as u64
    }
}

impl Solve for Solution {
    type Answer1 = u64;
    type Answer2 = u64;

    fn new(r: impl std::io::Read) -> Self {
        let mut buf = String::new();
        BufReader::new(r).read_to_string(&mut buf).ok();
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
