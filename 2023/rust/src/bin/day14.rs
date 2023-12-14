use aoc2023::Solve;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    platform: Vec<Vec<char>>,
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        Self {
            platform: BufReader::new(r)
                .lines()
                .map_while(Result::ok)
                .map(|line| line.chars().collect())
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let (rows, cols) = (self.platform.len(), self.platform[0].len());
        let mut ret = 0;
        for j in 0..cols {
            let mut load = cols as u32;
            for i in 0..rows {
                match self.platform[i][j] {
                    '#' => {
                        load = (rows - i - 1) as u32;
                    }
                    'O' => {
                        ret += load;
                        load -= 1;
                    }
                    _ => {}
                }
            }
        }
        ret
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
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(Solution::new(example_input()).part1(), 136);
    }
}
