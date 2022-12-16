use aoc2022::Solve;
use std::collections::{HashMap, VecDeque};
use std::io::{BufRead, BufReader, Read};

struct Solution {
    network: HashMap<String, (usize, Vec<String>)>,
}

impl Solution {
    fn max_total(
        v: &[(&String, usize)],
        dists: &[Vec<usize>],
        src: usize,
        remaining: usize,
        opened: &mut u32,
        memo: &mut HashMap<(usize, usize, u32), usize>,
    ) -> usize {
        if remaining == 0 {
            return 0;
        }
        if let Some(&max) = memo.get(&(src, remaining, *opened)) {
            return max;
        }
        let mut max = 0;
        for dst in 0..v.len() - 1 {
            if dst == src {
                continue;
            }
            let dist = dists[src][dst];
            if dist < remaining {
                max = max.max(Self::max_total(
                    v,
                    dists,
                    dst,
                    remaining - dist,
                    opened,
                    memo,
                ));
            }
        }
        if src < v.len() - 1 && (*opened & (1 << src)) == 0 {
            *opened |= 1 << src;
            max = max.max(
                v[src].1 * (remaining - 1)
                    + Self::max_total(v, dists, src, remaining - 1, opened, memo),
            );
            *opened &= !(1 << src);
        }
        memo.insert((src, remaining, *opened), max);
        max
    }
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;

    fn new(r: impl Read) -> Self {
        let mut network = HashMap::new();
        for line in BufReader::new(r).lines().filter_map(Result::ok) {
            if let Some((value, tunnels)) = line.split_once("; ") {
                network.insert(
                    value[6..8].to_string(),
                    (
                        value[23..].parse().unwrap(),
                        tunnels
                            .splitn(5, ' ')
                            .nth(4)
                            .map(|s| s.split(", ").map(String::from).collect())
                            .unwrap_or_default(),
                    ),
                );
            }
        }
        Self { network }
    }
    fn part1(&self) -> Self::Answer1 {
        let mut v = self
            .network
            .iter()
            .filter_map(|(valve, (flow_rate, _))| {
                Some((valve, *flow_rate)).filter(|(v, f)| *v == "AA" || *f > 0)
            })
            .collect::<Vec<_>>();
        v.sort();
        v.reverse();
        let mut dists = vec![vec![self.network.len(); v.len()]; v.len()];
        for (i, &(start, _)) in v.iter().enumerate() {
            let mut hm = HashMap::from([(start, 0)]);
            let mut vd = VecDeque::from([(start, 0)]);
            while let Some((valve, dist)) = vd.pop_front() {
                for tunnel in &self.network[valve].1 {
                    if !hm.contains_key(tunnel) {
                        hm.insert(tunnel, dist + 1);
                        vd.push_back((tunnel, dist + 1));
                    }
                }
            }
            for (j, (dst, _)) in v.iter().enumerate() {
                dists[i][j] = hm[dst];
            }
        }
        Self::max_total(&v, &dists, v.len() - 1, 30, &mut 0, &mut HashMap::new())
    }
    fn part2(&self) -> Self::Answer2 {
        todo!()
    }
}

fn main() {
    let solution = Solution::new(std::io::stdin().lock());
    println!("Part 1: {}", solution.part1());
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
}
