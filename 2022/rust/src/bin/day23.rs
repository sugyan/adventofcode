use aoc2022::Solve;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read};

const ADJACENTS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
const DIRECTIONS: [[(i32, i32); 3]; 4] = [
    [(-1, -1), (-1, 0), (-1, 1)],
    [(1, -1), (1, 0), (1, 1)],
    [(-1, -1), (0, -1), (1, -1)],
    [(-1, 1), (0, 1), (1, 1)],
];

struct DiffusionIter {
    elves: HashSet<(i32, i32)>,
    round: usize,
}

impl Iterator for DiffusionIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut hm = HashMap::new();
        for &(i, j) in &self.elves {
            if ADJACENTS
                .iter()
                .any(|&(di, dj)| self.elves.contains(&(i + di, j + dj)))
            {
                if let Some([_, (di, dj), _]) =
                    (0..4).map(|k| DIRECTIONS[(self.round + k) % 4]).find(|ds| {
                        ds.iter()
                            .all(|(di, dj)| !self.elves.contains(&(i + di, j + dj)))
                    })
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
            return None;
        }
        self.elves = hm
            .iter()
            .flat_map(|(k, v)| if v.len() == 1 { vec![*k] } else { v.clone() })
            .collect();
        self.round += 1;
        let (mut imin, mut imax, mut jmin, mut jmax) = (i32::MAX, i32::MIN, i32::MAX, i32::MIN);
        for &(i, j) in &self.elves {
            imin = imin.min(i);
            imax = imax.max(i);
            jmin = jmin.min(j);
            jmax = jmax.max(j);
        }
        Some(((imax - imin + 1) * (jmax - jmin + 1)) as usize - self.elves.len())
    }
}

#[derive(Clone)]
struct Solution {
    elves: HashSet<(i32, i32)>,
}

impl IntoIterator for Solution {
    type Item = usize;
    type IntoIter = DiffusionIter;

    fn into_iter(self) -> Self::IntoIter {
        DiffusionIter {
            elves: self.elves,
            round: 0,
        }
    }
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;

    fn new(r: impl Read) -> Self {
        Self {
            elves: (0..)
                .zip(BufReader::new(r).lines().filter_map(Result::ok))
                .flat_map(|(i, s)| {
                    (0..)
                        .zip(s.chars())
                        .filter_map(|(j, c)| match c {
                            '#' => Some((i, j)),
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                })
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.clone().into_iter().take(10).last().unwrap()
    }
    fn part2(&self) -> Self::Answer2 {
        self.clone().into_iter().count() + 1
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
