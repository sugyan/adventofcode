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

impl Solution {
    fn create_map(devices: &[Device]) -> HashMap<String, Vec<String>> {
        devices
            .iter()
            .map(|d| (d.name.clone(), d.outputs.clone()))
            .collect()
    }
    fn topological_sort(outputs: &HashMap<String, Vec<String>>) -> Vec<String> {
        let mut inputs = outputs
            .keys()
            .map(|key| (key.clone(), HashSet::new()))
            .collect::<HashMap<_, _>>();
        for (device, outputs) in outputs {
            for output in outputs {
                if let Some(s) = inputs.get_mut(output) {
                    s.insert(device);
                }
            }
        }
        let mut sorted = Vec::new();
        while !inputs.is_empty() {
            let ready = inputs
                .iter()
                .find_map(|(k, v)| if v.is_empty() { Some(k.clone()) } else { None })
                .unwrap();
            sorted.push(ready.clone());
            inputs.remove(&ready);
            for out in &outputs[&ready] {
                if let Some(s) = inputs.get_mut(out) {
                    s.remove(&ready);
                }
            }
        }
        sorted
    }
    fn count_paths(
        (src, dst): (&str, &str),
        outputs: &HashMap<String, Vec<String>>,
        sorted: &[String],
    ) -> u64 {
        let mut counts = [(src, 1)].into_iter().collect::<HashMap<_, _>>();
        for device in sorted {
            let count = counts.get(device.as_str()).cloned().unwrap_or(0);
            for out in &outputs[device] {
                *counts.entry(out).or_insert(0) += count;
            }
        }
        counts[dst]
    }
}

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = u64;
    type Answer2 = u64;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        let outputs = Self::create_map(&input.0);
        let sorted = Self::topological_sort(&outputs);
        Self::count_paths(("you", "out"), &outputs, &sorted)
    }

    fn part2(input: &Self::Input) -> Self::Answer2 {
        let outputs = Self::create_map(&input.0);
        let sorted = Self::topological_sort(&outputs);
        [
            Self::count_paths(("svr", "fft"), &outputs, &sorted)
                * Self::count_paths(("fft", "dac"), &outputs, &sorted)
                * Self::count_paths(("dac", "out"), &outputs, &sorted),
            Self::count_paths(("svr", "dac"), &outputs, &sorted)
                * Self::count_paths(("dac", "fft"), &outputs, &sorted)
                * Self::count_paths(("fft", "out"), &outputs, &sorted),
        ]
        .into_iter()
        .sum()
    }
}

fn main() -> Result<(), aoc2025::Error<Error>> {
    run::<Solution>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() -> Result<(), Error> {
        let input = r"
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
        .parse()?;
        assert_eq!(Solution::part1(&input), 5);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        let input = r"
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
"
        .trim_start()
        .parse()?;
        assert_eq!(Solution::part2(&input), 2);
        Ok(())
    }
}
