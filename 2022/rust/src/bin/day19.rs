use aoc2022::Solve;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

#[derive(Debug)]
struct Blueprint {
    costs: [[u32; 4]; 4],
    max: [u32; 4],
}

impl Blueprint {
    fn max_geodes(&self, minutes: u32) -> u32 {
        let mut max = 0;
        self.dfs([1, 0, 0, 0], [0; 4], minutes, &mut max);
        max
    }
    fn dfs(&self, robots: [u32; 4], resources: [u32; 4], minutes: u32, max: &mut u32) {
        if resources[3] + robots[3] * minutes + (minutes.max(1) - 1) * minutes / 2 < *max {
            return;
        }
        for (i, cost) in self.costs.iter().enumerate() {
            if i < 3 && resources[i] >= (self.max[i] - robots[i]) * minutes
                || (0..4).any(|j| cost[j] > 0 && robots[j] == 0)
            {
                continue;
            }
            let (mut next_robots, mut next_resources) = (robots, resources);
            let mut wait = 0;
            while wait < minutes && cost.iter().enumerate().any(|(j, &c)| next_resources[j] < c) {
                (0..4).for_each(|j| next_resources[j] += robots[j]);
                wait += 1;
            }
            if wait == minutes {
                *max = next_resources[3].max(*max);
            } else {
                for (j, c) in cost.iter().enumerate() {
                    next_resources[j] += robots[j];
                    next_resources[j] -= c;
                }
                next_robots[i] += 1;
                self.dfs(next_robots, next_resources, minutes - wait - 1, max);
            }
        }
    }
}

impl FromStr for Blueprint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s
            .split(' ')
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<_>>();
        let costs = [
            [v[0], 0, 0, 0],
            [v[1], 0, 0, 0],
            [v[2], v[3], 0, 0],
            [v[4], 0, v[5], 0],
        ];
        let max = [v[0].max(v[1]).max(v[2]).max(v[4]), v[3], v[5], 0];
        Ok(Blueprint { costs, max })
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
        (1..)
            .zip(&self.blueprints)
            .map(|(i, blurprint)| i * blurprint.max_geodes(24))
            .sum()
    }
    fn part2(&self) -> Self::Answer2 {
        self.blueprints
            .iter()
            .take(3)
            .map(|blueprint| blueprint.max_geodes(32))
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
