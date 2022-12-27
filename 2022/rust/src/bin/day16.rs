use aoc2022::Solve;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

#[derive(Debug)]
struct Report {
    valve: String,
    flow_rate: u32,
    tunnels: Vec<String>,
}

impl FromStr for Report {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once("; ")
            .map(|(v, t)| Self {
                valve: v[6..8].to_string(),
                flow_rate: v[23..].parse().unwrap(),
                tunnels: t
                    .splitn(5, ' ')
                    .nth(4)
                    .map(|s| s.split(", ").map(String::from).collect())
                    .unwrap_or_default(),
            })
            .ok_or(())
    }
}

struct Solution {
    target_map: Vec<(u32, Vec<u32>)>,
}

impl Solution {
    fn max_total(&self, src: usize, remaining: u32, target: u32) -> u32 {
        if remaining == 0 {
            return 0;
        }
        let (flow_rate, dsts) = &self.target_map[src];
        if target & (1 << src) != 0 {
            return flow_rate * (remaining - 1)
                + self.max_total(src, remaining - 1, target & !(1 << src));
        }
        let mut ret = 0;
        for (dst, &minutes) in dsts.iter().enumerate() {
            if dst != src && target & (1 << dst) != 0 && remaining > minutes {
                ret = ret.max(self.max_total(dst, remaining - minutes, target));
            }
        }
        ret
    }
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        let map = BufReader::new(r)
            .lines()
            .filter_map(Result::ok)
            .filter_map(|line| line.parse::<Report>().ok())
            .map(|r| (r.valve, (r.flow_rate, r.tunnels)))
            .collect::<HashMap<_, _>>();
        let mut targets = map
            .iter()
            .filter_map(|(valve, &(flow_rate, _))| {
                if flow_rate > 0 {
                    Some(valve.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let target2index = targets
            .iter()
            .enumerate()
            .map(|(i, valve)| (valve.clone(), i))
            .collect::<HashMap<_, _>>();
        targets.push(String::from("AA"));
        Self {
            target_map: targets
                .iter()
                .map(|src| {
                    let mut distances = vec![0; targets.len() - 1];
                    let mut visited = HashSet::new();
                    let mut vd = VecDeque::new();
                    vd.push_back((src.clone(), 0));
                    while let Some((dst, d)) = vd.pop_front() {
                        if visited.contains(&dst) {
                            continue;
                        }
                        visited.insert(dst.clone());
                        if let Some(&j) = target2index.get(&dst) {
                            distances[j] = d;
                        }
                        for t in &map[&dst].1 {
                            vd.push_back((t.clone(), d + 1));
                        }
                    }
                    (map[src].0, distances)
                })
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let start = self.target_map.len() - 1;
        self.max_total(start, 30, (1 << start) - 1)
    }
    fn part2(&self) -> Self::Answer2 {
        let start = self.target_map.len() - 1;
        let all = (1 << start) - 1;
        (0..=all)
            .map(|i| self.max_total(start, 26, i) + self.max_total(start, 26, all - i))
            .max()
            .unwrap()
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

    fn example_input() -> &'static [u8] {
        r"
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(1651, Solution::new(example_input()).part1());
    }

    #[test]
    fn part2() {
        assert_eq!(1707, Solution::new(example_input()).part2());
    }
}
