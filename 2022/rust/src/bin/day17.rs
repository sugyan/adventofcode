use std::io::{BufReader, Read};

use aoc2022::Solve;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

struct Solution {
    jet_patterns: Vec<Direction>,
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;

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
        let rock_patterns = [
            vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
            vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
            vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        ];
        let mut rocks = rock_patterns.into_iter().cycle();
        let mut jets = self.jet_patterns.iter().cycle();
        let mut chamber = vec![[false; 7]; 0];
        for _ in 0..2022 {
            if let Some(rock) = rocks.next() {
                let mut rock = rock
                    .iter()
                    .map(|(i, j)| (i + chamber.len() + 3, j + 2))
                    .collect::<Vec<_>>();
                chamber.extend(vec![[false; 7]; 7]);
                for jet in jets.by_ref() {
                    match jet {
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
            }
        }
        chamber.len()
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
>>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(3068, Solution::new(example_input()).part1());
    }
}
