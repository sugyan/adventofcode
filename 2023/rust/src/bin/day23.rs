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
    fn longest_steps(paths: &HashMap<Position, Vec<(Position, u32)>>, src: Position) -> u32 {
        let mut max = 0;
        if let Some(v) = paths.get(&src) {
            for (dst, steps) in v {
                max = max.max(steps + Self::longest_steps(paths, *dst));
            }
        } else {
            return 0;
        }
        max
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
        Self::longest_steps(&paths, (0, 1))
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
}
