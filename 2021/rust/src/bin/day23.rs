use aoc2021::Solve;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::fmt::Display;
use std::io::{BufRead, BufReader, Read};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Situation {
    diagram: Vec<Vec<char>>,
}

impl Situation {
    const ENERGY_MAP: [Option<u32>; 127] = {
        let mut map = [None; 127];
        map['A' as usize] = Some(1);
        map['B' as usize] = Some(10);
        map['C' as usize] = Some(100);
        map['D' as usize] = Some(1000);
        map
    };
    fn candidates(&self) -> Vec<(Situation, u32)> {
        let mut ret = Vec::new();
        let mut amphipods = Vec::new();
        for (i, row) in self.diagram.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if let Some(energy) = Self::ENERGY_MAP[*col as usize] {
                    if j == ((*col as usize) - 64) * 2 + 1
                        && (self.diagram[i + 1][j] == *col || self.diagram[i + 1][j] == '#')
                    {
                        continue;
                    }
                    amphipods.push(((i, j), *col, energy));
                }
            }
        }
        for ((i, j), c, energy) in amphipods {
            let mut visited = HashSet::new();
            let mut vd = VecDeque::from([((i, j), 0)]);
            while let Some((p, e)) = vd.pop_back() {
                for d in [(!0, 0), (0, !0), (0, 1), (1, 0)] {
                    let (ii, jj) = (p.0.wrapping_add(d.0), p.1.wrapping_add(d.1));
                    if self.diagram[ii][jj] != '.' || visited.contains(&(ii, jj)) {
                        continue;
                    }
                    visited.insert((ii, jj));
                    vd.push_back(((ii, jj), e + energy));
                    if self.can_move((i, j), (ii, jj), c) {
                        let mut diagram = self.diagram.clone();
                        diagram[i][j] = '.';
                        diagram[ii][jj] = c;
                        ret.push((Situation { diagram }, e + energy));
                    }
                }
            }
        }
        ret
    }
    fn can_move(&self, src: (usize, usize), dst: (usize, usize), c: char) -> bool {
        if [(1, 3), (1, 5), (1, 7), (1, 9)].contains(&dst) {
            return false;
        }
        if src.0 == 1 && dst.0 == 1 {
            return false;
        }
        if dst.0 > 1 {
            if dst.1 != ((c as usize) - 64) * 2 + 1 {
                return false;
            }
            if self.diagram[dst.0 + 1][dst.1] != c && self.diagram[dst.0 + 1][dst.1] != '#' {
                return false;
            }
        }
        true
    }
    fn is_finished(&self) -> bool {
        self.diagram[2][3] == 'A'
            && self.diagram[3][3] == 'A'
            && self.diagram[2][5] == 'B'
            && self.diagram[3][5] == 'B'
            && self.diagram[2][7] == 'C'
            && self.diagram[3][7] == 'C'
            && self.diagram[2][9] == 'D'
            && self.diagram[3][9] == 'D'
    }
}

impl Display for Situation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.diagram
            .iter()
            .try_for_each(|row| writeln!(f, "{}", row.iter().collect::<String>()))
    }
}

struct Solution {
    situation: Situation,
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        Self {
            situation: Situation {
                diagram: BufReader::new(r)
                    .lines()
                    .filter_map(Result::ok)
                    .map(|s| s.chars().collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
            },
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let mut hs = HashSet::new();
        let mut bh = BinaryHeap::from([(Reverse(0), self.situation.clone())]);
        while let Some((Reverse(total), min)) = bh.pop() {
            if min.is_finished() {
                return total;
            }
            if hs.contains(&min) {
                continue;
            }
            hs.insert(min.clone());
            for (situation, energy) in min.candidates() {
                bh.push((Reverse(total + energy), situation));
            }
        }
        unreachable!()
    }
    fn part2(&self) -> Self::Answer2 {
        unimplemented!()
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
}
