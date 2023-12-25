use aoc2023::Solve;
use itertools::{iproduct, Itertools};
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::io::{BufRead, BufReader, Read};

type Coordinate = (usize, usize, usize);

struct Solution {
    snapshots: Vec<(Coordinate, Coordinate)>,
}

impl Solution {
    fn drop_counts(&self) -> Vec<usize> {
        let (mut supportings, mut rev) = (HashMap::new(), HashMap::new());
        let mut grid = HashMap::new();
        for (i, &((x0, y0, z0), (x1, y1, z1))) in self
            .snapshots
            .iter()
            .sorted_by_cached_key(|((_, _, z0), (_, _, z1))| z0.min(z1))
            .enumerate()
        {
            let (mut m, mut v) = (0, Vec::new());
            for (x, y) in iproduct!(x0..=x1, y0..=y1) {
                if let Some(&(z, j)) = grid.get(&(x, y)) {
                    match m.cmp(&z) {
                        Ordering::Less => {
                            m = z;
                            v = vec![j];
                        }
                        Ordering::Equal => {
                            v.push(j);
                        }
                        Ordering::Greater => {}
                    }
                }
            }
            for (x, y, z) in iproduct!(x0..=x1, y0..=y1, z0..=z1) {
                grid.insert((x, y), (m + 1 + z - z0, i));
            }
            supportings.insert(i, v.iter().cloned().collect::<HashSet<_>>());
            for j in v {
                rev.entry(j).or_insert_with(BTreeSet::new).insert(i);
            }
        }
        (0..self.snapshots.len())
            .map(|i| {
                let mut drops = HashSet::from([i]);
                let mut bts = rev.get(&i).cloned().unwrap_or_default();
                while let Some(j) = bts.pop_first() {
                    if supportings[&j].is_subset(&drops) {
                        drops.insert(j);
                        if let Some(hs) = rev.get(&j) {
                            bts.extend(hs.iter().cloned());
                        }
                    }
                }
                drops.len() - 1
            })
            .collect()
    }
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;

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
        self.drop_counts().iter().filter(|&c| *c == 0).count()
    }
    fn part2(&self) -> Self::Answer2 {
        self.drop_counts().iter().sum()
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

    #[test]
    fn part2() {
        assert_eq!(Solution::new(example_input()).part2(), 7);
    }
}
