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

#[derive(Debug)]
enum Module {
    FlipFlop(bool),
    Conjunction(HashMap<String, Pulse>),
    Broadcast,
}

#[derive(Debug)]
struct State {
    modules: HashMap<String, (Module, Vec<String>)>,
}

impl State {
    fn new(configurations: &HashMap<String, Config>) -> Self {
        let mut modules = configurations
            .iter()
            .map(|(name, config)| {
                (
                    name.clone(),
                    (
                        match config.module_type {
                            ModuleType::FlipFlop => Module::FlipFlop(false),
                            ModuleType::Conjunction => Module::Conjunction(HashMap::new()),
                            ModuleType::Broadcast => Module::Broadcast,
                        },
                        config.destinations.clone(),
                    ),
                )
            })
            .collect::<HashMap<_, _>>();
        // collect inputs for conjunction modules
        for (name, config) in configurations {
            for destination in &config.destinations {
                if let Some((Module::Conjunction(module), _)) = modules.get_mut(destination) {
                    module.insert(name.clone(), Pulse::Low);
                }
            }
        }
        Self { modules }
    }
    fn push_button(&mut self) -> HashMap<String, [u32; 2]> {
        let mut counts = HashMap::new();
        let mut vd = VecDeque::from([(String::from("broadcaster"), Pulse::Low)]);
        while let Some((name, pulse)) = vd.pop_front() {
            counts.entry(name.clone()).or_insert([0, 0])[pulse as usize] += 1;
            if let Some((module, destinations)) = self.modules.get_mut(&name) {
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
                for destination in destinations.clone() {
                    vd.push_back((destination.clone(), pulse));
                    if let Some((Module::Conjunction(inputs), _)) =
                        self.modules.get_mut(&destination)
                    {
                        inputs.insert(name.clone(), pulse);
                    }
                }
            }
        }
        counts
    }
}

struct Solution {
    configuration: HashMap<String, Config>,
}

impl Solution {
    fn find_cycle(&self, target: &str) -> u64 {
        let mut state = State::new(&self.configuration);
        for i in 1.. {
            if state.push_button()[target][1] != 0 {
                return i;
            }
        }
        unreachable!()
    }
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u64;

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
        let total = (0..1000).fold([0, 0], |acc, _| {
            state.push_button().iter().fold(acc, |acc, (_, counts)| {
                [acc[0] + counts[0], acc[1] + counts[1]]
            })
        });
        total[0] * total[1]
    }
    fn part2(&self) -> Self::Answer2 {
        State::new(&self.configuration)
            .modules
            .iter()
            .find_map(|(_, (module, destinations))| {
                if destinations == &["rx"] {
                    if let Module::Conjunction(inputs) = module {
                        return Some(inputs.keys().cloned().collect::<Vec<_>>());
                    }
                }
                None
            })
            .expect("should have a module with `rx` as single destination")
            .iter()
            .map(|target| self.find_cycle(target))
            .product()
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
