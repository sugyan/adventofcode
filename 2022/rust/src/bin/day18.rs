use aoc2022::Solve;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::io::{BufRead, BufReader, Read};

struct Solution {
    cubes: HashSet<(usize, usize, usize)>,
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
                })
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.cubes.len() * 6
            - self
                .cubes
                .iter()
                .map(|&(x, y, z)| {
                    [(x + 1, y, z), (x, y + 1, z), (x, y, z + 1)]
                        .iter()
                        .filter(|p| self.cubes.contains(p))
                        .count()
                        * 2
                })
                .sum::<usize>()
    }
    fn part2(&self) -> Self::Answer2 {
        let maxs = self.cubes.iter().fold([0, 0, 0], |acc, &(x, y, z)| {
            [acc[0].max(x), acc[1].max(y), acc[2].max(z)]
        });
        let mut seen = vec![vec![vec![false; maxs[2] + 1]; maxs[1] + 1]; maxs[0] + 1];
        let mut vd = VecDeque::<[usize; 3]>::from([[0, 0, 0]]);
        let mut ret = self
            .cubes
            .iter()
            .map(|&(x, y, z)| {
                [x % maxs[0], y % maxs[1], z % maxs[2]]
                    .iter()
                    .filter(|&r| *r == 0)
                    .count()
            })
            .sum();
        while let Some([x, y, z]) = vd.pop_front() {
            if seen[x][y][z] {
                continue;
            }
            seen[x][y][z] = true;
            for &(dx, dy, dz) in &[
                (1, 0, 0),
                (0, 1, 0),
                (0, 0, 1),
                (!0, 0, 0),
                (0, !0, 0),
                (0, 0, !0),
            ] {
                let [xx, yy, zz] = [x.wrapping_add(dx), y.wrapping_add(dy), z.wrapping_add(dz)];
                if (0..=maxs[0]).contains(&xx)
                    && (0..=maxs[1]).contains(&yy)
                    && (0..=maxs[2]).contains(&zz)
                {
                    if self.cubes.contains(&(xx, yy, zz)) {
                        ret += 1;
                    } else {
                        vd.push_back([xx, yy, zz]);
                    }
                }
            }
        }
        ret
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
