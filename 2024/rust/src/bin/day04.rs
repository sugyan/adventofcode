use aoc2024::{run, Solve};
use std::io::{BufRead, BufReader, Read};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

struct Solution {
    letters: Vec<Vec<char>>,
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;
    type Error = Error;

    fn new<R>(r: R) -> Result<Self, Error>
    where
        R: Read,
    {
        Ok(Self {
            letters: BufReader::new(r)
                .lines()
                .map(|line| line.map_err(Error::Io).map(|line| line.chars().collect()))
                .collect::<Result<_, _>>()?,
        })
    }
    fn part1(&self) -> Self::Answer1 {
        let (rows, cols) = (self.letters.len(), self.letters[0].len());
        let mut ret = 0;
        for i in 0..rows {
            for j in 0..cols - 3 {
                let s = (0..4).map(|k| self.letters[i][j + k]).collect::<String>();
                if s == "XMAS" || s == "SAMX" {
                    ret += 1;
                }
            }
        }
        for i in 0..rows - 3 {
            for j in 0..cols {
                let s = (0..4).map(|k| self.letters[i + k][j]).collect::<String>();
                if s == "XMAS" || s == "SAMX" {
                    ret += 1;
                }
            }
        }
        for i in 0..rows - 3 {
            for j in 0..cols - 3 {
                let s = (0..4)
                    .map(|k| self.letters[i + k][j + k])
                    .collect::<String>();
                if s == "XMAS" || s == "SAMX" {
                    ret += 1;
                }
            }
        }
        for i in 3..rows {
            for j in 0..cols - 3 {
                let s = (0..4)
                    .map(|k| self.letters[i - k][j + k])
                    .collect::<String>();
                if s == "XMAS" || s == "SAMX" {
                    ret += 1;
                }
            }
        }
        ret
    }
    fn part2(&self) -> Self::Answer2 {
        let (rows, cols) = (self.letters.len(), self.letters[0].len());
        let mut ret = 0;
        for i in 1..rows - 1 {
            for j in 1..cols - 1 {
                if self.letters[i][j] == 'A'
                    && [
                        [self.letters[i - 1][j - 1], self.letters[i + 1][j + 1]]
                            .into_iter()
                            .collect::<String>(),
                        [self.letters[i - 1][j + 1], self.letters[i + 1][j - 1]]
                            .into_iter()
                            .collect::<String>(),
                    ]
                    .iter()
                    .all(|s| s == "MS" || s == "SM")
                {
                    ret += 1;
                }
            }
        }
        ret
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
