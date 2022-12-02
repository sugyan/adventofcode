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
            let mut moved = false;
            // east
            let mut next = vec![vec!['.'; c]; r];
            for i in 0..r {
                for j in 0..c {
                    if map[i][j] == '>' {
                        if map[i][(j + 1) % c] == '.' {
                            next[i][(j + 1) % c] = '>';
                            moved = true;
                        } else {
                            next[i][j] = '>';
                        }
                    }
                    if map[i][j] == 'v' {
                        next[i][j] = 'v';
                    }
                }
            }
            map = next;
            // south
            let mut next = vec![vec!['.'; c]; r];
            for j in 0..c {
                for i in 0..r {
                    if map[i][j] == '>' {
                        next[i][j] = '>';
                    }
                    if map[i][j] == 'v' {
                        if map[(i + 1) % r][j] == '.' {
                            next[(i + 1) % r][j] = 'v';
                            moved = true;
                        } else {
                            next[i][j] = 'v';
                        }
                    }
                }
            }
            map = next;

            if !moved {
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
....v..v.>
"[1..]
            .as_bytes()
    }

    #[test]
    fn example1() {
        assert_eq!(58, Solution::new(example_input()).part1());
    }
}
