use aoc2022::Solve;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::io::{BufRead, BufReader, Read};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate(usize, usize, usize);

impl TryFrom<Option<(usize, usize, usize)>> for Coordinate {
    type Error = ();

    fn try_from(value: Option<(usize, usize, usize)>) -> Result<Self, Self::Error> {
        if let Some((x, y, z)) = value {
            Ok(Self(x, y, z))
        } else {
            Err(())
        }
    }
}

impl Coordinate {
    fn adjacents(&self) -> HashSet<Coordinate> {
        HashSet::from([
            Coordinate(self.0.wrapping_add(!0), self.1, self.2),
            Coordinate(self.0.wrapping_add(1), self.1, self.2),
            Coordinate(self.0, self.1.wrapping_add(!0), self.2),
            Coordinate(self.0, self.1.wrapping_add(1), self.2),
            Coordinate(self.0, self.1, self.2.wrapping_add(!0)),
            Coordinate(self.0, self.1, self.2.wrapping_add(1)),
        ])
    }
}

struct Solution {
    cubes: HashSet<Coordinate>,
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;

    fn new(r: impl Read) -> Self {
        Self {
            cubes: BufReader::new(r)
                .lines()
                .filter_map(Result::ok)
                .filter_map(|line| {
                    line.split(',')
                        .filter_map(|s| s.parse().ok())
                        .collect_tuple()
                        .try_into()
                        .ok()
                })
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.cubes
            .iter()
            .map(|cube| cube.adjacents().difference(&self.cubes).count())
            .sum()
    }
    fn part2(&self) -> Self::Answer2 {
        let maxs = self.cubes.iter().fold([0, 0, 0], |acc, c| {
            [acc[0].max(c.0), acc[1].max(c.1), acc[2].max(c.2)]
        });
        let in_range = |c: &Coordinate| {
            (0..=maxs[0]).contains(&c.0)
                && (0..=maxs[1]).contains(&c.1)
                && (0..=maxs[2]).contains(&c.2)
        };

        let mut seen = vec![vec![vec![false; maxs[2] + 1]; maxs[1] + 1]; maxs[0] + 1];
        let mut vd = VecDeque::from([Coordinate(0, 0, 0)]);
        while let Some(c) = vd.pop_front() {
            for n in c.adjacents().difference(&self.cubes) {
                if in_range(n) && !seen[n.0][n.1][n.2] {
                    vd.push_back(*n);
                    seen[n.0][n.1][n.2] = true;
                }
            }
        }
        self.cubes
            .iter()
            .map(|cube| {
                cube.adjacents()
                    .iter()
                    .filter(|&n| !in_range(n) || seen[n.0][n.1][n.2])
                    .count()
            })
            .sum()
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
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(64, Solution::new(example_input()).part1());
    }

    #[test]
    fn part2() {
        assert_eq!(58, Solution::new(example_input()).part2());
    }
}
