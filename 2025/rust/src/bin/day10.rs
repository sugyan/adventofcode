use aoc2025::{Day, run};
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    ops::{BitOr, BitXor},
    str::FromStr,
};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("invalid input")]
    InvalidInput,
}

#[derive(Clone, Default, PartialEq, Eq, Hash)]
struct IndexSet(u32);

impl IndexSet {
    fn len(&self) -> u32 {
        self.0.count_ones()
    }
    fn contains(&self, index: usize) -> bool {
        (self.0 & (1 << index)) != 0
    }
    fn is_subset(&self, other: &Self) -> bool {
        (self.0 & other.0) == self.0
    }
    fn indices(&self) -> Vec<usize> {
        let (mut u, mut v) = (self.0, Vec::new());
        while u != 0 {
            v.push(u.trailing_zeros() as usize);
            u &= u - 1;
        }
        v
    }
}

impl BitOr<usize> for IndexSet {
    type Output = Self;

    fn bitor(self, rhs: usize) -> Self::Output {
        Self(self.0 | (1 << rhs))
    }
}

impl BitXor<usize> for IndexSet {
    type Output = Self;

    fn bitxor(self, rhs: usize) -> Self::Output {
        Self(self.0 ^ (1 << rhs))
    }
}

impl Debug for IndexSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.indices()).finish()
    }
}

#[derive(Debug)]
struct Machine {
    indicator_lights: Vec<bool>,
    wiring_schematics: Vec<Vec<usize>>,
    joltage_requirements: Vec<u32>,
}

impl Machine {
    fn fewest_presses_for_indicator_lights(&self) -> u32 {
        let target = self
            .indicator_lights
            .iter()
            .fold(0, |acc, b| (acc << 1) | if *b { 1 } else { 0 });
        let schematics = self
            .wiring_schematics
            .iter()
            .map(|schematic| schematic.iter().map(|x| 1 << *x).sum::<u32>())
            .collect_vec();
        let mut hs = schematics.iter().cloned().collect::<HashSet<_>>();
        let mut presses = 1;
        while !hs.contains(&target) {
            presses += 1;
            hs = hs
                .iter()
                .cartesian_product(&schematics)
                .map(|(a, b)| a ^ b)
                .collect();
        }
        presses
    }
    fn fewest_presses_for_joltage_requirements(&self) -> u32 {
        fn resolve(input: HashMap<IndexSet, u32>) -> Option<(HashMap<IndexSet, u32>, u32)> {
            if let Some((k, v)) = input.iter().find(|(k, _)| k.len() == 1) {
                let mut new_input = HashMap::new();
                for (key, value) in &input {
                    if key == k {
                        continue;
                    }
                    if (key.0 & k.0) != 0 {
                        let new_key = IndexSet(key.0 ^ k.0);
                        let new_value = value.checked_sub(*v)?;
                        if let Some(value) = input.get(&new_key)
                            && *value != new_value
                        {
                            return None;
                        }
                        new_input.insert(new_key, new_value);
                    } else {
                        new_input.insert(key.clone(), *value);
                    }
                }
                return resolve(new_input).map(|(input, current)| (input, current + v));
            }
            let mut v = Vec::new();
            for c in input.keys().combinations(2) {
                let (k0, k1) = (c[0], c[1]);
                if k0.is_subset(k1) {
                    v.push((IndexSet(k1.0 ^ k0.0), input[k1].checked_sub(input[k0])?));
                }
                if k1.is_subset(k0) {
                    v.push((IndexSet(k0.0 ^ k1.0), input[k0].checked_sub(input[k1])?));
                }
                if (k0.0 & k1.0) == 0 {
                    v.push((IndexSet(k0.0 | k1.0), input[k0] + input[k1]));
                }
            }
            v.retain(|(k, _)| !input.contains_key(k));
            if v.is_empty() {
                Some((input, 0))
            } else {
                resolve(input.into_iter().chain(v).collect())
            }
        }
        fn recursive((input, current): (HashMap<IndexSet, u32>, u32)) -> Option<u32> {
            let Some((k, v)) = input.iter().min_by_key(|(_, v)| *v) else {
                return Some(current);
            };
            let target = k.indices()[0];
            (0..=*v)
                .filter_map(|n| {
                    if let Some(resolved) = resolve(
                        input
                            .clone()
                            .into_iter()
                            .map(|(key, value)| {
                                if key.contains(target) {
                                    (key ^ target, value - n)
                                } else {
                                    (key, value)
                                }
                            })
                            .collect(),
                    ) && let Some(ret) = recursive(resolved)
                    {
                        Some(ret + n + current)
                    } else {
                        None
                    }
                })
                .min()
        }
        let init = self
            .joltage_requirements
            .iter()
            .enumerate()
            .map(|(i, req)| {
                (
                    self.wiring_schematics
                        .iter()
                        .enumerate()
                        .filter_map(|(j, schematics)| {
                            if schematics.contains(&i) {
                                Some(j)
                            } else {
                                None
                            }
                        })
                        .fold(IndexSet::default(), |acc, x| acc | x),
                    *req,
                )
            })
            .collect();
        let resolved = resolve(init).unwrap();
        recursive(resolved).unwrap()
    }
}

impl FromStr for Machine {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect_vec();
        let trim = |s: &str, (prefix, suffix): (char, char)| {
            s.strip_prefix(prefix)
                .and_then(|s| s.strip_suffix(suffix))
                .ok_or(Error::InvalidInput)
                .map(String::from)
        };
        let indicator_lights = trim(parts[0], ('[', ']'))?
            .chars()
            .rev()
            .map(|c| match c {
                '#' => Ok(true),
                '.' => Ok(false),
                _ => Err(Error::InvalidInput),
            })
            .try_collect()?;
        let wiring_schematics = parts[1..parts.len() - 1]
            .iter()
            .map(|part| {
                trim(part, ('(', ')'))?
                    .split(',')
                    .map(str::parse)
                    .try_collect()
                    .map_err(|_| Error::InvalidInput)
            })
            .try_collect()?;
        Ok(Self {
            indicator_lights,
            wiring_schematics,
            joltage_requirements: trim(parts[parts.len() - 1], ('{', '}'))?
                .split(',')
                .map(str::parse)
                .try_collect()?,
        })
    }
}

struct Input(Vec<Machine>);

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.lines().map(str::parse).try_collect()?))
    }
}

struct Solution;

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = u32;
    type Answer2 = u32;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        input
            .0
            .iter()
            .map(Machine::fewest_presses_for_indicator_lights)
            .sum()
    }

    fn part2(input: &Self::Input) -> Self::Answer2 {
        input
            .0
            .iter()
            .map(Machine::fewest_presses_for_joltage_requirements)
            .sum()
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
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"
        .trim_start()
        .parse()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::part1(&example_input()?), 7);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::part2(&example_input()?), 33);
        Ok(())
    }
}
