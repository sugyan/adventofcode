use aoc2022::Solve;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read};

struct Solution {
    elves: HashSet<(isize, isize)>,
}

impl Solution {
    fn diffuse_elves(&self, stop_10: bool) -> (usize, HashSet<(isize, isize)>) {
        let mut elves = self.elves.clone();
        let adjacents = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let directions = [
            [(-1, -1), (-1, 0), (-1, 1)],
            [(1, -1), (1, 0), (1, 1)],
            [(-1, -1), (0, -1), (1, -1)],
            [(-1, 1), (0, 1), (1, 1)],
        ];
        for round in 1.. {
            let mut hm = HashMap::new();
            for &(i, j) in &elves {
                if adjacents
                    .iter()
                    .any(|&(di, dj)| elves.contains(&(i + di, j + dj)))
                {
                    if let Some([_, (di, dj), _]) = directions
                        .iter()
                        .cycle()
                        .skip((round - 1) % 4)
                        .take(4)
                        .find(|ds| ds.iter().all(|(di, dj)| !elves.contains(&(i + di, j + dj))))
                    {
                        hm.entry((i + di, j + dj))
                            .or_insert_with(Vec::new)
                            .push((i, j));
                        continue;
                    }
                }
                hm.insert((i, j), vec![(i, j)]);
            }
            if hm.iter().all(|(k, v)| [*k] == v.as_slice()) {
                return (round, elves);
            }
            elves = hm
                .iter()
                .flat_map(|(k, v)| if v.len() == 1 { vec![*k] } else { v.clone() })
                .collect();
            if stop_10 && round == 10 {
                return (round, elves);
            }
        }
        unreachable!()
    }
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;

    fn new(r: impl Read) -> Self {
        Self {
            elves: BufReader::new(r)
                .lines()
                .filter_map(Result::ok)
                .enumerate()
                .flat_map(|(i, s)| {
                    s.chars()
                        .enumerate()
                        .filter_map(|(j, c)| match c {
                            '#' => Some((i as isize, j as isize)),
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                })
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let elves = self.diffuse_elves(true).1;
        let imin = *elves.iter().map(|(i, _)| i).min().unwrap();
        let imax = *elves.iter().map(|(i, _)| i).max().unwrap();
        let jmin = *elves.iter().map(|(_, j)| j).min().unwrap();
        let jmax = *elves.iter().map(|(_, j)| j).max().unwrap();
        ((imax - imin + 1) * (jmax - jmin + 1)) as usize - elves.len()
    }
    fn part2(&self) -> Self::Answer2 {
        self.diffuse_elves(false).0
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
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
        "[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(110, Solution::new(example_input()).part1());
    }

    #[test]
    fn part2() {
        assert_eq!(20, Solution::new(example_input()).part2());
    }
}
