use aoc2025::{Day, run};
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("invalid input")]
    InvalidInput,
}

#[derive(Debug)]
struct Device {
    name: String,
    outputs: Vec<String>,
}

impl FromStr for Device {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(": ")
            .ok_or(Error::InvalidInput)
            .map(|(name, outputs)| Self {
                name: name.to_string(),
                outputs: outputs.split(' ').map(String::from).collect(),
            })
    }
}

struct Input(Vec<Device>);

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
        let hm = input
            .0
            .iter()
            .map(|d| (&d.name, &d.outputs))
            .collect::<HashMap<_, _>>();
        let mut inputs = input
            .0
            .iter()
            .map(|d| (&d.name, HashSet::new()))
            .collect::<HashMap<_, _>>();
        for device in &input.0 {
            for output in &device.outputs {
                if let Some(s) = inputs.get_mut(&output) {
                    s.insert(&device.name);
                }
            }
        }
        let mut sorted = Vec::new();
        while !inputs.is_empty() {
            let ready = inputs
                .iter()
                .find_map(|(k, v)| if v.is_empty() { Some(*k) } else { None })
                .unwrap();
            sorted.push(ready);
            inputs.remove(ready);
            for out in hm[ready] {
                if let Some(s) = inputs.get_mut(out) {
                    s.remove(ready);
                }
            }
        }
        let mut counts = [("you", 1)].into_iter().collect::<HashMap<_, _>>();
        for device in sorted {
            let count = counts.get(device.as_str()).cloned().unwrap_or(0);
            for out in hm[device] {
                *counts.entry(out).or_insert(0) += count;
            }
        }
        counts["out"]
    }

    fn part2(_: &Self::Input) -> Self::Answer2 {
        todo!()
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
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
"
        .trim_start()
        .parse()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::part1(&example_input()?), 5);
        Ok(())
    }
}
