use aoc2024::{Day, run_day};
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};
use thiserror::Error;

const ADJACENTS: [(usize, usize); 4] = [(!0, 0), (1, 0), (0, !0), (0, 1)];
const DIRECTIONS_8: [(usize, usize); 8] = [
    (!0, 0),
    (!0, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, !0),
    (0, !0),
    (!0, !0),
];

#[derive(Error, Debug)]
enum Error {}

struct Input(HashMap<(usize, usize), u8>);

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .lines()
            .map(|line| line.bytes().collect_vec())
            .collect_vec();
        Ok(Self(
            map.iter()
                .enumerate()
                .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, u)| ((i, j), *u)))
                .collect(),
        ))
    }
}

struct Solution;

impl Solution {
    fn total_price<F>(input: &Input, f: F) -> usize
    where
        F: Fn(&Input, &[(usize, usize)], &u8) -> usize,
    {
        let mut sum = 0;
        let mut seen = HashSet::with_capacity(input.0.len());
        for (p, u) in &input.0 {
            if !seen.contains(p) {
                let area = Self::bfs(input, p, u, &mut seen);
                sum += area.len() * f(input, &area, u)
            }
        }
        sum
    }
    fn bfs(
        input: &Input,
        p: &(usize, usize),
        u: &u8,
        seen: &mut HashSet<(usize, usize)>,
    ) -> Vec<(usize, usize)> {
        let mut area = Vec::new();
        let mut vd = [*p].into_iter().collect::<VecDeque<_>>();
        seen.insert(*p);
        while let Some(p) = vd.pop_front() {
            area.push(p);
            for (di, dj) in ADJACENTS {
                let (i, j) = (p.0.wrapping_add(di), p.1.wrapping_add(dj));
                if input.0.get(&(i, j)) == Some(u) && !seen.contains(&(i, j)) {
                    seen.insert((i, j));
                    vd.push_back((i, j));
                }
            }
        }
        area
    }
    fn calculate_perimeter(input: &Input, area: &[(usize, usize)], u: &u8) -> usize {
        area.iter()
            .cartesian_product(ADJACENTS)
            .filter(|&((i, j), (di, dj))| {
                input.0.get(&(i.wrapping_add(di), j.wrapping_add(dj))) != Some(u)
            })
            .count()
    }
    fn count_corners(input: &Input, area: &[(usize, usize)], u: &u8) -> usize {
        area.iter()
            .flat_map(|(i, j)| {
                DIRECTIONS_8
                    .iter()
                    .map(|&(di, dj)| {
                        input.0.get(&(i.wrapping_add(di), j.wrapping_add(dj))) != Some(u)
                    })
                    .circular_tuple_windows()
                    .step_by(2)
            })
            .filter(|t| matches!(t, (true, _, true) | (false, true, false)))
            .count()
    }
}

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = usize;
    type Answer2 = usize;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        Self::total_price(input, Self::calculate_perimeter)
    }
    fn part2(input: &Self::Input) -> Self::Answer2 {
        Self::total_price(input, Self::count_corners)
    }
}

fn main() -> Result<(), aoc2024::Error<Error>> {
    run_day::<Solution>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input_1() -> Result<Input, Error> {
        r"
AAAA
BBCD
BBCC
EEEC
"
        .trim_start()
        .parse()
    }

    fn example_input_2() -> Result<Input, Error> {
        r"
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
"
        .trim_start()
        .parse()
    }

    fn example_input_3() -> Result<Input, Error> {
        r"
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"
        .trim_start()
        .parse()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::part1(&example_input_1()?), 140);
        assert_eq!(Solution::part1(&example_input_2()?), 772);
        assert_eq!(Solution::part1(&example_input_3()?), 1930);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::part2(&example_input_1()?), 80);
        assert_eq!(Solution::part2(&example_input_2()?), 436);
        assert_eq!(Solution::part2(&example_input_3()?), 1206);
        assert_eq!(
            Solution::part2(
                &r"
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
"
                .trim_start()
                .parse()?
            ),
            236
        );
        assert_eq!(
            Solution::part2(
                &r"
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
"
                .trim_start()
                .parse()?
            ),
            368
        );
        Ok(())
    }
}
