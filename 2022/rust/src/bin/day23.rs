use aoc2022::Solve;
use itertools::Itertools;
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

#[derive(Debug, Clone)]
struct Diffusion {
    elves: HashSet<(i32, i32)>,
    round: usize,
}

impl Diffusion {
    fn new(elves: HashSet<(i32, i32)>) -> Self {
        Self { elves, round: 0 }
    }
}

impl Iterator for Diffusion {
    type Item = HashSet<(i32, i32)>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut hm = HashMap::new();
        for &(i, j) in &self.elves {
            if ADJACENTS
                .iter()
                .any(|&(di, dj)| self.elves.contains(&(i + di, j + dj)))
            {
                if let Some([_, (di, dj), _]) = DIRECTIONS
                    .iter()
                    .cycle()
                    .skip(self.round % 4)
                    .take(4)
                    .find(|ds| {
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
        Some(self.elves.clone())
    }
}

struct Solution {
    elves: HashSet<(i32, i32)>,
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
        let elves = Diffusion::new(self.elves.clone()).take(10).last().unwrap();
        let (imin, imax) = elves.iter().map(|(i, _)| i).minmax().into_option().unwrap();
        let (jmin, jmax) = elves.iter().map(|(_, j)| j).minmax().into_option().unwrap();
        ((imax - imin + 1) * (jmax - jmin + 1)) as usize - elves.len()
    }
    fn part2(&self) -> Self::Answer2 {
        Diffusion::new(self.elves.clone()).count() + 1
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
