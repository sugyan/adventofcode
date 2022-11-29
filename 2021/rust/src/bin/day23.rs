use aoc2021::Solve;
use std::array;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::io::{BufRead, BufReader, Read};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Situation {
    positions: [Vec<(usize, usize)>; 4],
}

impl Situation {
    #[rustfmt::skip]
    const EMPTY_DIAGRAMS: [[[char; 13]; 7]; 2] = [
        [
            ['#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#'],
            ['#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
            ['#', '#', '#', '.', '#', '.', '#', '.', '#', '.', '#', '#', '#'],
            ['#', '#', '#', '.', '#', '.', '#', '.', '#', '.', '#', '#', '#'],
            ['#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#'],
            ['#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#'],
            ['#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#'],
        ],
        [
            ['#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#'],
            ['#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
            ['#', '#', '#', '.', '#', '.', '#', '.', '#', '.', '#', '#', '#'],
            ['#', '#', '#', '.', '#', '.', '#', '.', '#', '.', '#', '#', '#'],
            ['#', '#', '#', '.', '#', '.', '#', '.', '#', '.', '#', '#', '#'],
            ['#', '#', '#', '.', '#', '.', '#', '.', '#', '.', '#', '#', '#'],
            ['#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#'],
        ],
    ];
    fn candidates(&self, diagram_index: usize) -> Vec<(Situation, u32)> {
        let mut ret = Vec::new();
        let mut diagram = Self::EMPTY_DIAGRAMS[diagram_index];
        for (i, v) in self.positions.iter().enumerate() {
            for &(r, c) in v {
                diagram[r][c] = (i as u8 + b'A') as char;
            }
        }
        for (i, v) in self.positions.iter().enumerate() {
            for (j, &src) in v.iter().enumerate() {
                if src.0 > 1 && src.1 == i * 2 + 3 {
                    let c = diagram[src.0 + 1][src.1];
                    if c == diagram[src.0][src.1] || c == '#' {
                        continue;
                    }
                }
                let mut visited = [[false; 13]; 7];
                let mut vd = VecDeque::from([(src, 1)]);
                while let Some((p, e)) = vd.pop_back() {
                    for d in [(!0, 0), (0, !0), (0, 1), (1, 0)] {
                        let dst = (p.0.wrapping_add(d.0), p.1.wrapping_add(d.1));
                        if diagram[dst.0][dst.1] != '.' || visited[dst.0][dst.1] {
                            continue;
                        }
                        visited[dst.0][dst.1] = true;
                        vd.push_back((dst, e + 1));
                        if Self::can_move(src, dst, &diagram) {
                            let mut positions = self.positions.clone();
                            positions[i][j] = dst;
                            ret.push((Situation { positions }, e * [1, 10, 100, 1000][i]));
                        }
                    }
                }
            }
        }
        ret
    }
    fn can_move(src: (usize, usize), dst: (usize, usize), diagram: &[[char; 13]; 7]) -> bool {
        if [(1, 3), (1, 5), (1, 7), (1, 9)].contains(&dst) {
            return false;
        }
        if src.0 == 1 && dst.0 == 1 {
            return false;
        }
        if dst.0 > 1 {
            let c_src = diagram[src.0][src.1];
            if dst.1 != (c_src as u8 - b'A') as usize * 2 + 3 {
                return false;
            }
            let c = diagram[dst.0 + 1][dst.1];
            if c != c_src && c != '#' {
                return false;
            }
        }
        true
    }
    fn is_finished(&self) -> bool {
        self.positions
            .iter()
            .enumerate()
            .all(|(i, v)| v.iter().all(|p| p.0 > 1 && p.1 == i * 2 + 3))
    }
}

struct Solution {
    situation: Situation,
}

impl Solution {
    fn find_minimum(&self, situation: Situation, n: usize) -> u32 {
        let mut hs = HashSet::new();
        let mut bh = BinaryHeap::from([(Reverse(0), situation)]);
        while let Some((Reverse(total), min)) = bh.pop() {
            if min.is_finished() {
                return total;
            }
            if hs.contains(&min) {
                continue;
            }
            hs.insert(min.clone());
            for (situation, energy) in min.candidates(n) {
                bh.push((Reverse(total + energy), situation));
            }
        }
        unreachable!()
    }
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        let diagram = BufReader::new(r)
            .lines()
            .filter_map(Result::ok)
            .map(|s| s.bytes().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut positions = array::from_fn(|_| Vec::new());
        for (i, row) in diagram.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if matches!(col, b'A'..=b'D') {
                    positions[(*col - b'A') as usize].push((i, j));
                }
            }
        }
        Self {
            situation: Situation { positions },
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.find_minimum(self.situation.clone(), 0)
    }
    fn part2(&self) -> Self::Answer2 {
        let mut situation = self.situation.clone();
        situation.positions.iter_mut().for_each(|v| {
            v.iter_mut().for_each(|p| {
                if p.0 > 2 {
                    p.0 += 2;
                }
            })
        });
        situation.positions[0].push((3, 9));
        situation.positions[0].push((4, 7));
        situation.positions[1].push((3, 7));
        situation.positions[1].push((4, 5));
        situation.positions[2].push((3, 5));
        situation.positions[2].push((4, 9));
        situation.positions[3].push((3, 3));
        situation.positions[3].push((4, 3));
        self.find_minimum(situation, 1)
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
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########"[1..]
            .as_bytes()
    }

    #[test]
    fn example1() {
        assert_eq!(12521, Solution::new(example_input()).part1());
    }

    #[test]
    fn example2() {
        assert_eq!(44169, Solution::new(example_input()).part2());
    }
}
