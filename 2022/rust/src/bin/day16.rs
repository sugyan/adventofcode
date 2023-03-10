use aoc2022::Solve;
use itertools::Itertools;
use std::cmp::{Ordering, Reverse};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::ops::{BitAnd, Not};
use std::str::FromStr;

struct Valve {
    label: String,
    flow_rate: u32,
    tunnels: Vec<String>,
}

impl FromStr for Valve {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once("; ")
            .map(|(v, t)| Self {
                label: v[6..8].to_string(),
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

#[derive(Clone, Copy)]
struct BitSet(u64);

impl Iterator for BitSet {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 != 0 {
            let ret = self.0.trailing_zeros() as usize;
            self.0 &= self.0 - 1;
            Some(ret)
        } else {
            None
        }
    }
}

impl BitAnd for BitSet {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl Not for BitSet {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

struct Solution {
    valves: Vec<Valve>,
    dists: Vec<Vec<u32>>,
    flows: BitSet,
}

impl Solution {
    fn max_totals(&self, minutes: u32) -> Vec<u32> {
        let ones = self.flows.0.count_ones() as usize;
        let mut totals = vec![0; 1 << ones];
        self.search(ones, BitSet(0), minutes, 0, &mut totals);
        totals
    }
    fn search(&self, i: usize, opened: BitSet, minutes: u32, total: u32, totals: &mut Vec<u32>) {
        if let Some(max) = totals.get_mut(opened.0 as usize) {
            *max = total.max(*max)
        }
        for j in self.flows & !opened {
            let remain = minutes.saturating_sub(self.dists[i][j] + 1);
            if remain > 0 {
                self.search(
                    j,
                    BitSet(opened.0 | (1 << j)),
                    remain,
                    total + self.valves[j].flow_rate * remain,
                    totals,
                );
            }
        }
    }
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        let valves = BufReader::new(r)
            .lines()
            .filter_map(Result::ok)
            .filter_map(|line| line.parse::<Valve>().ok())
            .sorted_unstable_by(|va, vb| match vb.flow_rate.cmp(&va.flow_rate) {
                Ordering::Equal => va.label.cmp(&vb.label),
                ord => ord,
            })
            .collect_vec();
        let label2index = valves
            .iter()
            .enumerate()
            .map(|(i, valve)| (valve.label.clone(), i))
            .collect::<HashMap<_, _>>();
        let mut dists = vec![vec![u32::MAX; valves.len()]; valves.len()];
        for (i, valve) in valves.iter().enumerate() {
            dists[i][i] = 0;
            for t in &valve.tunnels {
                if let Some(&j) = label2index.get(t) {
                    dists[i][j] = 1;
                }
            }
        }
        for k in 0..valves.len() {
            for i in 0..valves.len() {
                for j in 0..valves.len() {
                    dists[i][j] = dists[i][j].min(dists[i][k].saturating_add(dists[k][j]));
                }
            }
        }
        let flows = BitSet(valves.iter().enumerate().fold(0, |acc, (i, valve)| {
            acc | (u64::from(valve.flow_rate > 0) << i)
        }));
        Self {
            valves,
            dists,
            flows,
        }
    }
    fn part1(&self) -> Self::Answer1 {
        *self.max_totals(30).iter().max().unwrap()
    }
    fn part2(&self) -> Self::Answer2 {
        let v = self
            .max_totals(26)
            .into_iter()
            .enumerate()
            .filter(|&(_, t)| t > 0)
            .sorted_by_cached_key(|&(_, t)| Reverse(t))
            .collect_vec();
        let mut best = v[0].1;
        for &(i0, t0) in &v {
            if t0 * 2 < best {
                break;
            }
            for &(i1, t1) in &v {
                if i0 & i1 == 0 {
                    best = best.max(t0 + t1);
                }
            }
        }
        best
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
