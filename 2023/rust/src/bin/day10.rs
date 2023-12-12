use aoc2023::Solve;
use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    tiles: Vec<Vec<char>>,
    loops: HashSet<(usize, usize)>,
}

fn find_loop(
    graph: &Vec<Vec<Vec<(usize, usize)>>>,
    start: (usize, usize),
) -> Option<HashSet<(usize, usize)>> {
    let (rows, cols) = (graph.len(), graph[0].len());
    for (di, dj) in [(!0, 0), (1, 0), (0, !0), (0, 1)] {
        let i = start.0.wrapping_add(di);
        let j = start.1.wrapping_add(dj);
        if (0..rows).contains(&i) && (0..cols).contains(&j) {
            let mut visited = HashSet::new();
            if dfs(graph, (i, j), start, &mut visited) {
                return Some(visited);
            }
        }
    }
    None
}

fn dfs(
    graph: &Vec<Vec<Vec<(usize, usize)>>>,
    (i, j): (usize, usize),
    prev: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
) -> bool {
    visited.insert((i, j));
    for &p in &graph[i][j] {
        if p != prev && (visited.contains(&p) || dfs(graph, p, (i, j), visited)) {
            return true;
        }
    }
    false
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        let tiles = BufReader::new(r)
            .lines()
            .map_while(Result::ok)
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let (rows, cols) = (tiles.len(), tiles[0].len());
        let mut start = None;
        let mut graph = vec![vec![Vec::with_capacity(2); cols]; rows];
        for i in 0..rows {
            for j in 0..cols {
                if tiles[i][j] == 'S' {
                    start = Some((i, j));
                }
                if i < rows - 1
                    && matches!(tiles[i][j], '|' | '7' | 'F' | 'S')
                    && matches!(tiles[i + 1][j], '|' | 'L' | 'J' | 'S')
                {
                    graph[i][j].push((i + 1, j));
                    graph[i + 1][j].push((i, j));
                }
                if j < cols - 1
                    && matches!(tiles[i][j], '-' | 'L' | 'F' | 'S')
                    && matches!(tiles[i][j + 1], '-' | 'J' | '7' | 'S')
                {
                    graph[i][j].push((i, j + 1));
                    graph[i][j + 1].push((i, j));
                }
            }
        }

        let loops = find_loop(&graph, start.expect("should have start")).expect("should have loop");
        Self { tiles, loops }
    }
    fn part1(&self) -> Self::Answer1 {
        self.loops.len() / 2
    }
    fn part2(&self) -> Self::Answer2 {
        let mut answer = 0;
        for (i, row) in self.tiles.iter().enumerate() {
            for j in 0..row.len() {
                if self.loops.contains(&(i, j)) {
                    continue;
                }
                if (if row[..j].contains(&'S') {
                    (j + 1..row.len()).collect::<Vec<_>>()
                } else {
                    (0..j).collect::<Vec<_>>()
                })
                .iter()
                .filter_map(|&j| {
                    if self.loops.contains(&(i, j)) && self.tiles[i][j] != '-' {
                        Some(self.tiles[i][j])
                    } else {
                        None
                    }
                })
                .fold((0, None), |(count, prev), c| {
                    (
                        count
                            + i32::from(
                                c == '|'
                                    || (prev == Some('L') && c == '7')
                                    || (prev == Some('F') && c == 'J'),
                            ),
                        Some(c),
                    )
                })
                .0 % 2
                    == 1
                {
                    answer += 1;
                }
            }
        }
        answer
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

    fn example_input_3() -> &'static [u8] {
        r"
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
"[1..]
            .as_bytes()
    }

    fn example_input_4() -> &'static [u8] {
        r"
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
"[1..]
            .as_bytes()
    }

    fn example_input_5() -> &'static [u8] {
        r"
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(Solution::new(example_input_1()).part1(), 4);
        assert_eq!(Solution::new(example_input_2()).part1(), 8);
    }

    #[test]
    fn part2() {
        assert_eq!(Solution::new(example_input_3()).part2(), 4);
        assert_eq!(Solution::new(example_input_4()).part2(), 8);
        assert_eq!(Solution::new(example_input_5()).part2(), 10);
    }
}
