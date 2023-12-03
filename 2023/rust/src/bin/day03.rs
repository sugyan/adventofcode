use aoc2023::Solve;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    schematic: Vec<Vec<char>>,
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        Self {
            schematic: BufReader::new(r)
                .lines()
                .map_while(Result::ok)
                .map(|s| s.chars().collect())
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let (rows, cols) = (self.schematic.len(), self.schematic[0].len());
        let mut ret = 0;
        for (i, row) in self.schematic.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if !c.is_ascii_digit() || (j != 0 && row[j - 1].is_ascii_digit()) {
                    continue;
                }
                let (mut number, mut len) = (u32::from(*c as u8 - b'0'), 1);
                while j + len < cols && self.schematic[i][j + len].is_ascii_digit() {
                    number = number * 10 + u32::from(row[j + len] as u8 - b'0');
                    len += 1;
                }
                let mut is_part = false;
                for ii in (i.max(1) - 1)..=(i + 1).min(rows - 1) {
                    for jj in (j.max(1) - 1)..=(j + len).min(cols - 1) {
                        if !self.schematic[ii][jj].is_ascii_digit() && self.schematic[ii][jj] != '.'
                        {
                            is_part = true;
                        }
                    }
                }
                if is_part {
                    ret += number;
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
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(Solution::new(example_input()).part1(), 4361);
    }
}
