use aoc2021::Solve;
use std::io::{BufRead, BufReader, Read};

type Memo = Vec<Vec<Vec<Vec<[Option<[u64; 2]>; 2]>>>>;

struct Solution {
    starting_positions: Vec<u8>,
}

impl Solution {
    fn helper(p: [usize; 2], s: [usize; 2], player: usize, memo: &mut Memo) -> [u64; 2] {
        if let Some(wins) = memo[p[0]][p[1]][s[0]][s[1]][player] {
            return wins;
        }
        let mut ret = [0, 0];
        for (d, n) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
            let score = (p[player] + d) % 10 + 1;
            if s[player] + score > 20 {
                ret[player] += n;
            } else {
                (if player == 0 {
                    Self::helper([score - 1, p[1]], [s[0] + score, s[1]], 1, memo)
                } else {
                    Self::helper([p[0], score - 1], [s[0], s[1] + score], 0, memo)
                })
                .iter()
                .enumerate()
                .for_each(|(i, &w)| {
                    ret[i] += w * n;
                });
            }
        }
        memo[p[0]][p[1]][s[0]][s[1]][player] = Some(ret);
        ret
    }
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u64;

    fn new(r: impl Read) -> Self {
        Self {
            starting_positions: BufReader::new(r)
                .lines()
                .filter_map(Result::ok)
                .map(|s| s.split(": ").last().unwrap().parse::<u8>().unwrap() - 1)
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let mut positions = self.starting_positions.clone();
        let mut scores = vec![0; positions.len()];
        for i in (0..).step_by(3) {
            let j = (i / 3) % 2;
            positions[j] = (positions[j] + ((i * 3 + 6) % 10) as u8) % 10;
            scores[j] += (positions[j] % 10 + 1) as u32;
            if scores[j] >= 1000 {
                return scores[1 - j] * (i + 3) as u32;
            }
        }
        unreachable!()
    }
    fn part2(&self) -> Self::Answer2 {
        let positions = [
            self.starting_positions[0] as usize,
            self.starting_positions[1] as usize,
        ];
        let wins = Self::helper(
            positions,
            [0, 0],
            0,
            &mut vec![vec![vec![vec![[None; 2]; 21]; 21]; 10]; 10],
        );
        *wins.iter().max().unwrap()
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
Player 1 starting position: 4
Player 2 starting position: 8
"[1..]
            .as_bytes()
    }

    #[test]
    fn example1() {
        assert_eq!(739_785, Solution::new(example_input()).part1());
    }

    #[test]
    fn example2() {
        assert_eq!(444_356_092_776_315, Solution::new(example_input()).part2());
    }
}
