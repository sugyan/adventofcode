use aoc2024::{Day, run_day};
use std::{collections::HashMap, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {}

struct Input(HashMap<(i32, i32), u8>);

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .map(|line| line.bytes().collect())
            .collect::<Vec<Vec<_>>>();
        Ok(Self(
            grid.iter()
                .enumerate()
                .flat_map(|(i, row)| {
                    row.iter()
                        .enumerate()
                        .map(move |(j, &c)| ((i as i32, j as i32), c))
                })
                .collect(),
        ))
    }
}

struct Solution;

impl Solution {
    fn search_word(input: &Input, (i, j): (i32, i32), checks: &[((i32, i32), u8)]) -> bool {
        checks
            .iter()
            .all(|((di, dj), u)| input.0.get(&(i + di, j + dj)) == Some(u))
    }
}

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = usize;
    type Answer2 = usize;

    fn part1(input: &Self::Input) -> Self::Answer1 {
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
        input
            .0
            .iter()
            .filter_map(|(&p, u)| {
                if *u == b'X' {
                    Some(
                        directions
                            .into_iter()
                            .filter(|checks| Self::search_word(input, p, checks))
                            .count(),
                    )
                } else {
                    None
                }
            })
            .sum()
    }
    fn part2(input: &Self::Input) -> Self::Answer2 {
        let ms0 = [((-1, -1), b'M'), ((1, 1), b'S')];
        let sm0 = [((-1, -1), b'S'), ((1, 1), b'M')];
        let ms1 = [((1, -1), b'M'), ((-1, 1), b'S')];
        let sm1 = [((1, -1), b'S'), ((-1, 1), b'M')];
        input
            .0
            .iter()
            .filter(|&(&p, u)| {
                *u == b'A'
                    && (Self::search_word(input, p, &ms0) || Self::search_word(input, p, &sm0))
                    && (Self::search_word(input, p, &ms1) || Self::search_word(input, p, &sm1))
            })
            .count()
    }
}

fn main() -> Result<(), aoc2024::Error<Error>> {
    run_day::<Solution>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> Result<Input, Error> {
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
"
        .trim_start()
        .parse()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::part1(&example_input()?), 18);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::part2(&example_input()?), 9);
        Ok(())
    }
}
