use aoc2024::{run, Solve};
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

struct Solution {
    letters: HashMap<(i32, i32), u8>,
}

impl Solution {
    fn search_word(&self, (i, j): (i32, i32), checks: &[((i32, i32), u8)]) -> bool {
        checks.iter().all(|((di, dj), u)| {
            self.letters
                .get(&(i + di, j + dj))
                .map_or(false, |v| v == u)
        })
    }
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;
    type Error = Error;

    fn new<R>(r: R) -> Result<Self, Error>
    where
        R: Read,
    {
        let grid = BufReader::new(r)
            .lines()
            .map(|line| {
                line.map_err(Error::Io)
                    .map(|line| line.bytes().collect::<Vec<_>>())
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            letters: grid
                .iter()
                .enumerate()
                .flat_map(|(i, row)| {
                    row.iter()
                        .enumerate()
                        .map(move |(j, &c)| ((i as i32, j as i32), c))
                })
                .collect(),
        })
    }
    fn part1(&self) -> Self::Answer1 {
        let directions = [
            [((1, 0), b'M'), ((2, 0), b'A'), ((3, 0), b'S')],
            [((1, 1), b'M'), ((2, 2), b'A'), ((3, 3), b'S')],
            [((0, 1), b'M'), ((0, 2), b'A'), ((0, 3), b'S')],
            [((-1, 1), b'M'), ((-2, 2), b'A'), ((-3, 3), b'S')],
            [((-1, 0), b'M'), ((-2, 0), b'A'), ((-3, 0), b'S')],
            [((-1, -1), b'M'), ((-2, -2), b'A'), ((-3, -3), b'S')],
            [((0, -1), b'M'), ((0, -2), b'A'), ((0, -3), b'S')],
            [((1, -1), b'M'), ((2, -2), b'A'), ((3, -3), b'S')],
        ];
        self.letters
            .iter()
            .filter_map(|(&p, u)| {
                if *u == b'X' {
                    Some(
                        directions
                            .into_iter()
                            .filter(|checks| self.search_word(p, checks))
                            .count(),
                    )
                } else {
                    None
                }
            })
            .sum()
    }
    fn part2(&self) -> Self::Answer2 {
        let ms0 = [((-1, -1), b'M'), ((1, 1), b'S')];
        let sm0 = [((-1, -1), b'S'), ((1, 1), b'M')];
        let ms1 = [((1, -1), b'M'), ((-1, 1), b'S')];
        let sm1 = [((1, -1), b'S'), ((-1, 1), b'M')];
        self.letters
            .iter()
            .filter(|&(&p, u)| {
                *u == b'A'
                    && (self.search_word(p, &ms0) || self.search_word(p, &sm0))
                    && (self.search_word(p, &ms1) || self.search_word(p, &sm1))
            })
            .count()
    }
}

fn main() -> Result<(), Error> {
    run::<Solution>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static [u8] {
        r"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part1(), 18);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part2(), 9);
        Ok(())
    }
}
