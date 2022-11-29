use aoc2021::Solve;
use std::io::{BufRead, BufReader};

struct Solution {
    map: Vec<Vec<char>>,
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl std::io::Read) -> Self {
        Self {
            map: BufReader::new(r)
                .lines()
                .filter_map(Result::ok)
                .map(|s| s.chars().collect())
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let mut map = self.map.clone();
        let (r, c) = (map.len(), map[0].len());
        for i in 1.. {
            let mut global_moved = false;
            #[allow(clippy::needless_range_loop)]
            for i in 0..r {
                let c0 = map[i][0];
                let mut moved = false;
                for j in 0..c - 1 {
                    if !moved && map[i][j] == '>' && map[i][j + 1] == '.' {
                        map[i][j] = '.';
                        map[i][j + 1] = '>';
                        moved = true;
                    } else {
                        moved = false;
                    }
                    global_moved |= moved;
                }
                if !moved && map[i][c - 1] == '>' && c0 == '.' {
                    map[i][c - 1] = '.';
                    map[i][0] = '>';
                    global_moved = true;
                }
            }
            for j in 0..c {
                let c0 = map[0][j];
                let mut moved = false;
                for i in 0..r - 1 {
                    if !moved && map[i][j] == 'v' && map[i + 1][j] == '.' {
                        map[i][j] = '.';
                        map[i + 1][j] = 'v';
                        moved = true;
                    } else {
                        moved = false;
                    }
                    global_moved |= moved;
                }
                if !moved && map[r - 1][j] == 'v' && c0 == '.' {
                    map[r - 1][j] = '.';
                    map[0][j] = 'v';
                    global_moved = true;
                }
            }
            if !global_moved {
                return i;
            }
        }
        unreachable!()
    }
    fn part2(&self) -> Self::Answer2 {
        unreachable!()
    }
}

fn main() {
    let solution = Solution::new(std::io::stdin().lock());
    println!("Part 1: {}", solution.part1());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static [u8] {
        r"
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>"[1..]
            .as_bytes()
    }

    #[test]
    fn example1() {
        assert_eq!(58, Solution::new(example_input()).part1());
    }
}
