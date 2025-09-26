use aoc2024::{Day, run_day};
use itertools::{Either, Itertools};
use std::{collections::HashMap, iter, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {}

struct Input {
    map_size: (usize, usize),
    antennas: HashMap<u8, Vec<(usize, usize)>>,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .lines()
            .map(|line| line.bytes().collect_vec())
            .collect_vec();
        let mut antennas = HashMap::new();
        for (i, row) in map.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if col != &b'.' {
                    antennas.entry(*col).or_insert_with(Vec::new).push((i, j));
                }
            }
        }
        Ok(Self {
            map_size: (map.len(), map[0].len()),
            antennas,
        })
    }
}

struct Solution;

impl Solution {
    fn count_antinodes(input: &Input, all_harmonics: bool) -> usize {
        input
            .antennas
            .values()
            .flat_map(|v| {
                v.iter().permutations(2).flat_map(|c| {
                    let (di, dj) = (c[1].0.wrapping_sub(c[0].0), c[1].1.wrapping_sub(c[0].1));
                    let step = move |(i, j): (usize, usize)| {
                        let (ni, nj) = (i.wrapping_add(di), j.wrapping_add(dj));
                        (ni < input.map_size.0 && nj < input.map_size.1).then_some((ni, nj))
                    };
                    let it = iter::successors(Some(*c[1]), move |p| step(*p));
                    if all_harmonics {
                        Either::Left(it)
                    } else {
                        Either::Right(step(*c[1]).into_iter())
                    }
                })
            })
            .unique()
            .count()
    }
}

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = usize;
    type Answer2 = usize;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        Self::count_antinodes(input, false)
    }
    fn part2(input: &Self::Input) -> Self::Answer2 {
        Self::count_antinodes(input, true)
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
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"
        .trim_start()
        .parse()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::part1(&example_input()?), 14);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::part2(&example_input()?), 34);
        Ok(())
    }
}
