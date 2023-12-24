use aoc2023::Solve;
use itertools::{iproduct, Itertools};
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read};

type Coordinate = (usize, usize, usize);

struct Solution {
    snapshots: Vec<(Coordinate, Coordinate)>,
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        Self {
            snapshots: BufReader::new(r)
                .lines()
                .map_while(Result::ok)
                .map(|line| {
                    let coordinates = line.split_once('~').expect("should be valid line");
                    let to_xyz = |s: &str| {
                        s.split(',')
                            .map(|s| s.parse().expect("should be valid number"))
                            .collect_tuple()
                            .expect("should be valid coordinates")
                    };
                    (to_xyz(coordinates.0), to_xyz(coordinates.1))
                })
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let mut grid = HashMap::new();
        for (i, &((x0, y0, z0), (x1, y1, z1))) in self
            .snapshots
            .iter()
            .sorted_by_cached_key(|((_, _, z0), (_, _, z1))| z0.min(z1))
            .enumerate()
        {
            let mut brick = iproduct!(x0..=x1, y0..=y1, z0..=z1).collect_vec();
            while brick
                .iter()
                .all(|&(x, y, z)| z > 0 && !grid.contains_key(&(x, y, z - 1)))
            {
                brick.iter_mut().for_each(|(_, _, z)| *z -= 1);
            }
            for &(x, y, z) in &brick {
                grid.insert((x, y, z), i);
            }
        }
        let mut supported = HashMap::new();
        for (&(x, y, z), i) in &grid {
            if let Some(&j) = grid.get(&(x, y, z + 1)) {
                if j != *i {
                    supported.entry(j).or_insert_with(HashSet::new).insert(*i);
                }
            }
        }
        let exclude = supported
            .values()
            .filter_map(|s| {
                if s.len() == 1 {
                    Some(s.iter().next().unwrap())
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>();
        grid.values()
            .filter(|i| !exclude.contains(i))
            .unique()
            .count()
    }
    fn part2(&self) -> Self::Answer2 {
        todo!()
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
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(Solution::new(example_input()).part1(), 5);
    }
}
