use aoc2023::Solve;
use std::collections::{HashMap, VecDeque};
use std::io::{BufRead, BufReader, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High = 0,
    Low = 1,
}

#[derive(Debug)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcast,
}

#[derive(Debug)]
struct Config {
    module_type: ModuleType,
    destinations: Vec<String>,
}

struct Solution {
    configuration: HashMap<String, Config>,
}

#[derive(Debug)]
enum Module {
    FlipFlop(bool),
    Conjunction(HashMap<String, Pulse>),
    Broadcast,
}

#[derive(Debug)]
struct State {
    modules: HashMap<String, Module>,
}

impl State {
    fn new(configurations: &HashMap<String, Config>) -> Self {
        let mut modules = configurations
            .iter()
            .map(|(name, config)| match config.module_type {
                ModuleType::FlipFlop => (name.clone(), Module::FlipFlop(false)),
                ModuleType::Conjunction => (name.clone(), Module::Conjunction(HashMap::new())),
                ModuleType::Broadcast => (name.clone(), Module::Broadcast),
            })
            .collect::<HashMap<_, _>>();
        for (name, config) in configurations {
            for destination in &config.destinations {
                if let Some(Module::Conjunction(module)) = modules.get_mut(destination) {
                    module.insert(name.clone(), Pulse::Low);
                }
            }
        }
        Self { modules }
    }
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        Self {
            configuration: BufReader::new(r)
                .lines()
                .map_while(Result::ok)
                .map(|line| {
                    let (label, destinations) =
                        line.split_once(" -> ").expect("should be valid line");
                    let (module_type, name) = if label == "broadcaster" {
                        (ModuleType::Broadcast, label.into())
                    } else if let Some(name) = label.strip_prefix('%') {
                        (ModuleType::FlipFlop, name.into())
                    } else if let Some(name) = label.strip_prefix('&') {
                        (ModuleType::Conjunction, name.into())
                    } else {
                        unreachable!();
                    };
                    (
                        name,
                        Config {
                            module_type,
                            destinations: destinations.split(", ").map(String::from).collect(),
                        },
                    )
                })
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let mut state = State::new(&self.configuration);
        let mut counts = [0, 0];
        for _ in 0..1000 {
            let mut vd = VecDeque::from([(String::from("broadcaster"), Pulse::Low)]);
            while let Some((name, pulse)) = vd.pop_front() {
                counts[pulse as usize] += 1;
                if let Some(module) = state.modules.get_mut(&name) {
                    let pulse = match module {
                        Module::FlipFlop(on) if pulse == Pulse::Low => {
                            *on = !*on;
                            if *on {
                                Pulse::High
                            } else {
                                Pulse::Low
                            }
                        }
                        Module::Conjunction(inputs) => {
                            if inputs.values().all(|&p| p == Pulse::High) {
                                Pulse::Low
                            } else {
                                Pulse::High
                            }
                        }
                        Module::Broadcast => pulse,
                        _ => continue,
                    };
                    for destination in &self.configuration[&name].destinations {
                        vd.push_back((destination.clone(), pulse));
                        if let Some(Module::Conjunction(inputs)) =
                            state.modules.get_mut(destination)
                        {
                            inputs.insert(name.clone(), pulse);
                        }
                    }
                }
            }
        }
        counts[0] * counts[1]
    }
    fn part2(&self) -> Self::Answer2 {
        todo!()
    }
}

fn main() {
    let solution = Solution::new(std::io::stdin().lock());
    println!("Part 1: {}", solution.part1());
    println!("Part 2: {}", solution.part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input_1() -> &'static [u8] {
        r"
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
"[1..]
            .as_bytes()
    }

    fn example_input_2() -> &'static [u8] {
        r"
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(Solution::new(example_input_1()).part1(), 32_000_000);
        assert_eq!(Solution::new(example_input_2()).part1(), 11_687_500);
    }
}
