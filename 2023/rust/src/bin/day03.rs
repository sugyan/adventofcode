use aoc2023::{run, Solve};
use itertools::{iproduct, Itertools};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};

type Point = (usize, usize);

struct Solution {
    part_numbers: HashMap<(char, Point), Vec<(u32, Point)>>,
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        let schematic = BufReader::new(r)
            .lines()
            .map_while(Result::ok)
            .map(|s| s.bytes().collect_vec())
            .collect_vec();
        let (rows, cols) = (schematic.len(), schematic[0].len());
        // collect numbers and their positions
        let mut numbers = HashMap::new();
        for (i, row) in schematic.iter().enumerate() {
            let (mut p, mut number) = (None, 0);
            for (j, b) in row.iter().enumerate() {
                (p, number) = if b.is_ascii_digit() {
                    (
                        Some(*p.get_or_insert((i, j))),
                        number * 10 + u32::from(b - b'0'),
                    )
                } else {
                    (None, 0)
                };
                if let Some(p) = p {
                    numbers.insert(p, number);
                }
            }
        }
        // search symbols adjacent to numbers, and store them in a map
        let mut part_numbers = HashMap::new();
        for (&(i, j), n) in &numbers {
            for p in iproduct!(
                (i.max(1) - 1)..=(i + 1).min(rows - 1),
                (j.max(1) - 1)..=(j + n.ilog10() as usize + 1).min(cols - 1)
            )
            .filter(|&(i, j)| !schematic[i][j].is_ascii_digit() && schematic[i][j] != b'.')
            {
                part_numbers
                    .entry((schematic[p.0][p.1] as char, p))
                    .or_insert_with(Vec::new)
                    .push((*n, (i, j)));
            }
        }
        Self { part_numbers }
    }
    fn part1(&self) -> Self::Answer1 {
        self.part_numbers
            .iter()
            .flat_map(|(_, v)| v)
            .unique()
            .map(|(n, _)| n)
            .sum()
    }
    fn part2(&self) -> Self::Answer2 {
        self.part_numbers
            .iter()
            .filter_map(|((c, _), v)| {
                if *c == '*' && v.len() == 2 {
                    Some(v[0].0 * v[1].0)
                } else {
                    None
                }
            })
            .sum()
    }
}

fn main() {
    run(&Solution::new(std::io::stdin().lock()));
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
