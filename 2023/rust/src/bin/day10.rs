use aoc2023::Solve;
use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    tiles: Vec<Vec<char>>,
}

fn dfs(
    graph: &Vec<Vec<Vec<(usize, usize)>>>,
    (i, j): (usize, usize),
    prev: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
) -> Option<usize> {
    visited.insert((i, j));
    for &p in &graph[i][j] {
        if p != prev {
            if visited.contains(&p) {
                return Some(visited.len());
            } else {
                match dfs(graph, p, (i, j), visited) {
                    None => {}
                    s => return s,
                }
            }
        }
    }
    None
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        Self {
            tiles: BufReader::new(r)
                .lines()
                .map_while(Result::ok)
                .map(|line| line.chars().collect())
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let (rows, cols) = (self.tiles.len(), self.tiles[0].len());
        let mut graph = vec![vec![Vec::with_capacity(2); cols]; rows];
        #[allow(clippy::needless_range_loop)]
        for i in 0..rows {
            for j in 0..cols - 1 {
                if matches!(self.tiles[i][j], '-' | 'L' | 'F' | 'S')
                    && matches!(self.tiles[i][j + 1], '-' | 'J' | '7' | 'S')
                {
                    graph[i][j].push((i, j + 1));
                    graph[i][j + 1].push((i, j));
                }
            }
        }
        for i in 0..rows - 1 {
            for j in 0..cols {
                if matches!(self.tiles[i][j], '|' | '7' | 'F' | 'S')
                    && matches!(self.tiles[i + 1][j], '|' | 'L' | 'J' | 'S')
                {
                    graph[i][j].push((i + 1, j));
                    graph[i + 1][j].push((i, j));
                }
            }
        }
        let start = self
            .tiles
            .iter()
            .enumerate()
            .find_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .find_map(|(j, &c)| if c == 'S' { Some((i, j)) } else { None })
            })
            .expect("should have start");
        for (di, dj) in [(!0, 0), (1, 0), (0, !0), (0, 1)] {
            let i = start.0.wrapping_add(di);
            let j = start.1.wrapping_add(dj);
            if (0..rows).contains(&i) && (0..cols).contains(&j) {
                if let Some(len) = dfs(&graph, (i, j), start, &mut HashSet::new()) {
                    return len / 2;
                }
            }
        }
        unreachable!();
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

    fn example_input_1() -> &'static [u8] {
        r"
-L|F7
7S-7|
L|7||
-L-J|
L|-JF
"[1..]
            .as_bytes()
    }

    fn example_input_2() -> &'static [u8] {
        r"
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(Solution::new(example_input_1()).part1(), 4);
        assert_eq!(Solution::new(example_input_2()).part1(), 8);
    }
}
