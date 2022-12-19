use aoc2022::Solve;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

#[derive(Debug)]
struct Blueprint([[u32; 3]; 4]);

impl Blueprint {
    fn max_geodes(
        &self,
        minutes: u32,
        resources: [u32; 4],
        robots: [u32; 4],
        memo: &mut HashMap<(u32, [u32; 4], [u32; 4]), u32>,
    ) -> u32 {
        if minutes == 0 {
            return resources[3];
        }
        if let Some(max) = memo.get(&(minutes, resources, robots)) {
            return *max;
        }
        let mut ret = 0;
        for (i, costs) in self.0.iter().enumerate() {
            if costs
                .iter()
                .enumerate()
                .all(|(j, cost)| resources[j] >= *cost)
            {
                let mut next_robots = robots;
                next_robots[i] += 1;
                ret = ret.max(self.max_geodes(
                    minutes - 1,
                    [
                        resources[0] + robots[0] - costs[0],
                        resources[1] + robots[1] - costs[1],
                        resources[2] + robots[2] - costs[2],
                        resources[3] + robots[3],
                    ],
                    next_robots,
                    memo,
                ));
            }
        }
        ret = ret.max(self.max_geodes(
            minutes - 1,
            [
                resources[0] + robots[0],
                resources[1] + robots[1],
                resources[2] + robots[2],
                resources[3] + robots[3],
            ],
            robots,
            memo,
        ));
        memo.insert((minutes, resources, robots), ret);
        ret
    }
}

impl FromStr for Blueprint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s
            .split(' ')
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<_>>();
        Ok(Blueprint([
            [v[0], 0, 0],
            [v[1], 0, 0],
            [v[2], v[3], 0],
            [v[4], 0, v[5]],
        ]))
    }
}

struct Solution {
    blueprints: Vec<Blueprint>,
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        Solution {
            blueprints: BufReader::new(r)
                .lines()
                .filter_map(Result::ok)
                .filter_map(|s| s.parse().ok())
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let mut ret = 0;
        for (i, blurprint) in (1..).zip(&self.blueprints) {
            let max_geodes = blurprint.max_geodes(24, [0; 4], [1, 0, 0, 0], &mut HashMap::new());
            ret += i * max_geodes;
        }
        ret
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
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(33, Solution::new(example_input()).part1());
    }
}
