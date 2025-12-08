use aoc2025::{Day, run};
use itertools::Itertools;
use std::str::FromStr;
use thiserror::Error;

#[cfg(test)]
const CONNECTIONS: usize = 10;
#[cfg(not(test))]
const CONNECTIONS: usize = 1000;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
    #[error("invalid position")]
    InvalidPosition,
}

struct Input(Vec<(u64, u64, u64)>);

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|line| {
                    line.split(',')
                        .collect_tuple()
                        .ok_or(Error::InvalidPosition)
                        .and_then(|(x, y, z)| Ok((x.parse()?, y.parse()?, z.parse()?)))
                })
                .try_collect()?,
        ))
    }
}

struct Solution;

impl Solution {
    fn closest_pairs(input: &Input) -> Vec<(usize, usize)> {
        input
            .0
            .iter()
            .enumerate()
            .combinations(2)
            .map(|combination| {
                let ((i, pi), (j, pj)) = (combination[0], combination[1]);
                (
                    pi.0.abs_diff(pj.0).pow(2)
                        + pi.1.abs_diff(pj.1).pow(2)
                        + pi.2.abs_diff(pj.2).pow(2),
                    (i, j),
                )
            })
            .sorted()
            .map(|(_, pair)| pair)
            .collect()
    }
    fn circuit_size(graph: &[Vec<usize>], start: usize, seen: &mut [bool]) -> usize {
        let mut size = 0;
        let mut stack = vec![start];
        while let Some(j) = stack.pop() {
            if seen[j] {
                continue;
            }
            seen[j] = true;
            size += 1;
            for &k in &graph[j] {
                if !seen[k] {
                    stack.push(k);
                }
            }
        }
        size
    }
}

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = usize;
    type Answer2 = u64;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        let pairs = Self::closest_pairs(input);
        let mut graph = vec![Vec::new(); input.0.len()];
        for &(i, j) in pairs.iter().take(CONNECTIONS) {
            graph[i].push(j);
            graph[j].push(i);
        }
        let mut seen = vec![false; input.0.len()];
        let mut sizes = Vec::new();
        for i in 0..input.0.len() {
            if !seen[i] {
                sizes.push(Self::circuit_size(&graph, i, &mut seen));
            }
        }
        sizes.iter().sorted().rev().take(3).product()
    }

    fn part2(input: &Self::Input) -> Self::Answer2 {
        let pairs = Self::closest_pairs(input);
        let mut graph = vec![Vec::new(); input.0.len()];
        for &(i, j) in &pairs {
            graph[i].push(j);
            graph[j].push(i);
            if Solution::circuit_size(&graph, 0, &mut vec![false; input.0.len()]) == input.0.len() {
                return input.0[i].0 * input.0[j].0;
            }
        }
        unreachable!()
    }
}

fn main() -> Result<(), aoc2025::Error<Error>> {
    run::<Solution>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> Result<Input, Error> {
        r"
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"
        .trim_start()
        .parse()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::part1(&example_input()?), 40);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::part2(&example_input()?), 25272);
        Ok(())
    }
}
