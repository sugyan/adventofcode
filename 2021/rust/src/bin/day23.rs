use aoc2021::Solve;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::io::{BufRead, BufReader, Read};

const MAP: [[u32; 11]; 16] = {
    let mut map = [[0; 11]; 16];
    let mut room = 0;
    while room < 16 {
        let src = (room / 4, room % 4 * 2 + 2);
        let mut hallway = 0;
        while hallway < 11 {
            let (mut i, mut j) = src;
            while i > 0 {
                i -= 1;
                map[room][hallway] |= 1 << (i * 4 + j / 2 - 1)
            }
            while j > hallway {
                map[room][hallway] |= 1 << (j + 16);
                j -= 1;
            }
            while j < hallway {
                map[room][hallway] |= 1 << (j + 16);
                j += 1;
            }
            hallway += 1;
        }
        room += 1;
    }
    map
};
const MASKS: [[u32; 16]; 2] = [
    [
        0x0000_0010,
        0x0000_0020,
        0x0000_0040,
        0x0000_0080,
        0xffff_ffff,
        0xffff_ffff,
        0xffff_ffff,
        0xffff_ffff,
        0x0000_0000,
        0x0000_0000,
        0x0000_0000,
        0x0000_0000,
        0x0000_0000,
        0x0000_0000,
        0x0000_0000,
        0x0000_0000,
    ],
    [
        0x0000_1110,
        0x0000_2220,
        0x0000_4440,
        0x0000_8880,
        0x0000_1100,
        0x0000_2200,
        0x0000_4400,
        0x0000_8800,
        0x0000_1000,
        0x0000_2000,
        0x0000_4000,
        0x0000_8000,
        0xffff_ffff,
        0xffff_ffff,
        0xffff_ffff,
        0xffff_ffff,
    ],
];

// (room)
// lower bits: ........ ........ fedcba98 76543210
// #############
// #...........#
// ###0#1#2#3###
//   #4#5#6#7#
//   #8#9#a#b#
//   #c#d#e#f#
//   #########
//
// (hallway)
// upper bits: .....a98 76543210 ........ ........
// #############
// #0123456789a#
// ###.#.#.#.###
//   #.#.#.#.#
//   #.#.#.#.#
//   #.#.#.#.#
//   #########
struct Solution {
    positions: [u32; 4],
}

impl Solution {
    const ENERGY_PER_STEPS: [u32; 4] = [1, 10, 100, 1000];
    fn candidate_positions(&self, positions: &[u32; 4], mask: usize) -> Vec<([u32; 4], u32)> {
        let occupied = positions.iter().fold(0, |acc, &v| acc | v);
        let mut ret = Vec::new();
        for (i, u) in positions.iter().enumerate() {
            #[allow(clippy::needless_range_loop)]
            // room -> hallway
            for room in 0..16 {
                if u & (1 << room) != 0 && (room % 4 != i || MASKS[mask][room] & u == 0) {
                    for hallway in (0..11).filter(|h| !matches!(h, 2 | 4 | 6 | 8)) {
                        let path = MAP[room][hallway] | (1 << (16 + hallway));
                        if path & occupied == 0 {
                            let mut next = *positions;
                            next[i] = u & !(1 << room) | 1 << (16 + hallway);
                            ret.push((next, path.count_ones() * Self::ENERGY_PER_STEPS[i]));
                        }
                    }
                }
            }
            // hallway -> room
            for hallway in 0..11 {
                if u & (1 << (16 + hallway)) != 0 {
                    for room in (0..4).map(|j| j * 4 + i) {
                        let path = MAP[room][hallway] | (1 << room);
                        if path & occupied == 0 && MASKS[mask][room] & u != 0 {
                            let mut next = *positions;
                            next[i] = u & !(1 << (16 + hallway)) | 1 << room;
                            ret.push((next, path.count_ones() * Self::ENERGY_PER_STEPS[i]));
                        }
                    }
                }
            }
        }
        ret
    }
    fn least_total_energy(&self, mut positions: [u32; 4], unfold: bool) -> u32 {
        if unfold {
            // finish.iter_mut().for_each(|u| *u |= *u << 8);
            let inserts = [0x0480, 0x0240, 0x0820, 0x0110];
            for (i, u) in positions.iter_mut().enumerate() {
                *u = (*u & 0xf0) << 8 | *u & 0x0f | inserts[i];
            }
        }
        let finish: [u32; 4] = [
            [0x0011, 0x0022, 0x0044, 0x0088],
            [0x1111, 0x2222, 0x4444, 0x8888],
        ][unfold as usize];
        let mut hs = HashSet::new();
        let mut bh = BinaryHeap::from([(Reverse(0), positions)]);
        while let Some((Reverse(total), min)) = bh.pop() {
            if hs.contains(&min) {
                continue;
            }
            hs.insert(min);
            if min == finish {
                return total;
            }
            for (p, e) in self.candidate_positions(&min, unfold.into()) {
                bh.push((Reverse(total + e), p));
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
        let mut positions = [0; 4];
        for (i, row) in diagram.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if matches!(col, b'A'..=b'D') {
                    positions[(*col - b'A') as usize] |= 1_u32 << ((i - 2) * 4 + j / 2 - 1);
                }
            }
        }
        Self { positions }
    }
    fn part1(&self) -> Self::Answer1 {
        self.least_total_energy(self.positions, false)
    }
    fn part2(&self) -> Self::Answer2 {
        self.least_total_energy(self.positions, true)
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
  #########
"[1..]
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
