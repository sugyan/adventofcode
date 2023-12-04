use aoc2023::Solve;
use itertools::Itertools;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};

type Symbol = (u8, (usize, usize));

struct Solution {
    numbers: Vec<(u32, Vec<Symbol>)>,
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        let schematic = BufReader::new(r)
            .lines()
            .map_while(Result::ok)
            .map(|s| s.bytes().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let (rows, cols) = (schematic.len(), schematic[0].len());
        let mut numbers = Vec::new();
        for (i, row) in schematic.iter().enumerate() {
            for (j, b) in row.iter().enumerate() {
                if !b.is_ascii_digit() || (j != 0 && row[j - 1].is_ascii_digit()) {
                    continue;
                }
                let (mut number, mut len) = (u32::from(b - b'0'), 1);
                while j + len < cols && schematic[i][j + len].is_ascii_digit() {
                    number = number * 10 + u32::from(row[j + len] - b'0');
                    len += 1;
                }
                let symbols = ((i.max(1) - 1)..=(i + 1).min(rows - 1))
                    .cartesian_product((j.max(1) - 1)..=(j + len).min(cols - 1))
                    .filter_map(|(i, j)| {
                        let b = schematic[i][j];
                        if !b.is_ascii_digit() && b != b'.' {
                            Some((b, (i, j)))
                        } else {
                            None
                        }
                    })
                    .collect();
                numbers.push((number, symbols));
            }
        }
        Self { numbers }
    }
    fn part1(&self) -> Self::Answer1 {
        self.numbers
            .iter()
            .filter_map(|(number, symbols)| {
                if symbols.is_empty() {
                    None
                } else {
                    Some(number)
                }
            })
            .sum()
    }
    fn part2(&self) -> Self::Answer2 {
        let mut gears = HashMap::new();
        for (number, symbols) in &self.numbers {
            for (c, (i, j)) in symbols {
                if *c == b'*' {
                    gears.entry((i, j)).or_insert_with(Vec::new).push(*number);
                }
            }
        }
        gears
            .values()
            .filter_map(|v| {
                if v.len() == 2 {
                    Some(v[0] * v[1])
                } else {
                    None
                }
            })
            .sum()
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

    #[test]
    fn part2() {
        assert_eq!(Solution::new(example_input()).part2(), 467_835);
    }
}
