use aoc2024::{Day, run_day};
use itertools::Itertools;
use std::{
    cell::OnceCell,
    cmp::Ordering,
    collections::{HashMap, HashSet},
    str::FromStr,
};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
    #[error("invalid input")]
    InvalidInput,
    #[error("invalid line")]
    InvalidLine,
}

type AnalyzedResult = Vec<(bool, Vec<u32>)>;

struct Input {
    rules: HashMap<u32, HashSet<u32>>,
    updates: Vec<Vec<u32>>,
    cell: OnceCell<AnalyzedResult>,
}

impl Input {
    fn analyzed(&self) -> &AnalyzedResult {
        self.cell.get_or_init(|| {
            self.updates
                .iter()
                .map(|update| {
                    let sorted = update
                        .iter()
                        .copied()
                        .sorted_by(|a, b| {
                            if self.rules.get(a).is_some_and(|set| set.contains(b)) {
                                Ordering::Less
                            } else {
                                Ordering::Greater
                            }
                        })
                        .collect_vec();
                    (sorted == *update, sorted)
                })
                .collect()
        })
    }
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect_vec();
        let (rule_lines, update_lines) = lines
            .split(|s| s.is_empty())
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
            .collect();
        Ok(Self {
            rules,
            updates,
            cell: OnceCell::new(),
        })
    }
}

struct Solution;

impl Solution {
    fn sum_of_middle_page_numbers(input: &Input, condition: bool) -> u32 {
        input
            .analyzed()
            .iter()
            .filter_map(|(is_sorted, sorted)| {
                if *is_sorted == condition {
                    Some(sorted[sorted.len() / 2])
                } else {
                    None
                }
            })
            .sum()
    }
}

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = u32;
    type Answer2 = u32;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        Self::sum_of_middle_page_numbers(input, true)
    }
    fn part2(input: &Self::Input) -> Self::Answer2 {
        Self::sum_of_middle_page_numbers(input, false)
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
        .trim_start()
        .parse()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::part1(&example_input()?), 143);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::part2(&example_input()?), 123);
        Ok(())
    }
}
