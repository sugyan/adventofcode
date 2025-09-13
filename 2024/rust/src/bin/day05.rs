use aoc2024::{Solve, run};
use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader, Read},
};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
    #[error("invalid input")]
    InvalidInput,
    #[error("invalid line")]
    InvalidLine,
}

struct Solution {
    rules: HashMap<u32, HashSet<u32>>,
    updates: Vec<(Vec<u32>, bool)>,
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;
    type Error = Error;

    fn new<R>(r: R) -> Result<Self, Error>
    where
        R: Read,
    {
        let lines = BufReader::new(r).lines().collect::<Result<Vec<_>, _>>()?;
        let (rule_lines, update_lines) = lines
            .split(String::is_empty)
            .collect_tuple()
            .ok_or(Error::InvalidInput)?;
        let mut rules = HashMap::new();
        for rule in rule_lines.iter().map(|line| {
            line.split('|')
                .map(|s| s.parse().map_err(Error::Parse))
                .collect::<Result<Vec<u32>, _>>()
                .and_then(|v| v.into_iter().collect_tuple().ok_or(Error::InvalidLine))
        }) {
            let (before, after) = rule?;
            rules
                .entry(before)
                .or_insert_with(HashSet::new)
                .insert(after);
        }
        let updates = update_lines
            .iter()
            .map(|line| {
                line.split(',')
                    .map(|s| s.parse().map_err(Error::Parse))
                    .collect()
            })
            .collect::<Result<Vec<Vec<_>>, _>>()?
            .into_iter()
            .map(|update| {
                let sorted =
                    update.is_sorted_by(|a, b| rules.get(a).is_some_and(|set| set.contains(b)));
                (update, sorted)
            })
            .collect();
        Ok(Self { rules, updates })
    }
    fn part1(&self) -> Self::Answer1 {
        self.updates
            .iter()
            .filter(|(_, sorted)| *sorted)
            .map(|(update, _)| update[update.len() / 2])
            .sum()
    }
    fn part2(&self) -> Self::Answer2 {
        self.updates
            .iter()
            .filter(|(_, sorted)| !*sorted)
            .map(|(update, _)| {
                update
                    .iter()
                    .cloned()
                    .sorted_by(|a, b| {
                        if self.rules.get(a).is_some_and(|set| set.contains(b)) {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    })
                    .collect_vec()[update.len() / 2]
            })
            .sum()
    }
}

fn main() -> Result<(), Error> {
    run::<Solution>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static [u8] {
        &r"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"
        .as_bytes()[1..]
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part1(), 143);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part2(), 123);
        Ok(())
    }
}
