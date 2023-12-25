use aoc2023::Solve;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read};

type Position = (usize, usize);

struct Solution {
    map: Vec<Vec<char>>,
}

impl Solution {
    fn calculate_paths(&self) -> HashMap<Position, Vec<(Position, u32)>> {
        let (rows, cols) = (self.map.len(), self.map[0].len());
        let mut paths = HashMap::new();
        let mut done = HashSet::new();
        let mut stack = vec![((0, 1), (1_usize, 1_usize))];
        while let Some((src, dst)) = stack.pop() {
            done.insert(src);
            let mut prev = src;
            let (mut i, mut j) = dst;
            for steps in 1.. {
                let nexts = [(!0, 0), (1, 0), (0, !0), (0, 1)]
                    .iter()
                    .filter_map(|&(di, dj)| {
                        let (i, j) = (i.wrapping_add(di), j.wrapping_add(dj));
                        if (i, j) != prev
                            && (0..rows).contains(&i)
                            && (0..cols).contains(&j)
                            && self.map[i][j] != '#'
                        {
                            Some((i, j))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                if nexts.len() == 1 {
                    prev = (i, j);
                    (i, j) = nexts[0];
                } else {
                    paths
                        .entry(src)
                        .or_insert_with(Vec::new)
                        .push(((i, j), steps));
                    for (ni, nj) in nexts {
                        if !done.contains(&(i, j))
                            && ((ni > i && self.map[ni][nj] == 'v')
                                || (ni < i && self.map[ni][nj] == '^')
                                || (nj > j && self.map[ni][nj] == '>')
                                || (nj < j && self.map[ni][nj] == '<'))
                        {
                            stack.push(((i, j), (ni, nj)));
                        }
                    }
                    break;
                }
            }
        }
        paths
    }
    fn longest_steps(&self, paths: &HashMap<Position, Vec<(Position, u32)>>) -> u32 {
        fn dfs(
            paths: &HashMap<Position, Vec<(Position, u32)>>,
            src: Position,
            end: Position,
            used: &mut HashSet<Position>,
        ) -> Option<u32> {
            let mut max = None;
            if let Some(v) = paths.get(&src) {
                for (dst, steps) in v {
                    if used.contains(dst) {
                        continue;
                    }
                    used.insert(*dst);
                    if let Some(ret) = dfs(paths, *dst, end, used) {
                        max = Some(max.map_or(steps + ret, |m: u32| m.max(steps + ret)));
                    }
                    used.remove(dst);
                }
            } else if src == end {
                return Some(0);
            } else {
                return None;
            }
            max
        }

        let (rows, cols) = (self.map.len(), self.map[0].len());
        dfs(
            paths,
            (0, 1),
            (rows - 1, cols - 2),
            &mut HashSet::from([(0, 1)]),
        )
        .expect("should have a longest path")
    }
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        Self {
            map: BufReader::new(r)
                .lines()
                .map_while(Result::ok)
                .map(|s| s.chars().collect())
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let paths = self.calculate_paths();
        self.longest_steps(&paths)
    }
    fn part2(&self) -> Self::Answer2 {
        let (rows, cols) = (self.map.len(), self.map[0].len());
        let mut paths = self.calculate_paths();
        let entries = paths
            .iter()
            .map(|(&k, v)| (k, v.clone()))
            .collect::<Vec<_>>();
        for (src, v) in &entries {
            for &(dst, steps) in v {
                if dst == (rows - 1, cols - 2) {
                    continue;
                }
                if let Some(v) = paths.get_mut(&dst) {
                    v.push((*src, steps));
                }
            }
        }
        self.longest_steps(&paths)
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
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(Solution::new(example_input()).part1(), 94);
    }

    #[test]
    fn part2() {
        assert_eq!(Solution::new(example_input()).part2(), 154);
    }
}
