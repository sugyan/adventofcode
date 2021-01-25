use std::collections::HashSet;
use std::io::{BufRead, BufReader};

struct Solution {
    active: HashSet<(i32, i32, i32, i32)>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        let mut active = HashSet::new();
        for (i, row) in inputs.iter().enumerate() {
            for (j, col) in row.chars().enumerate() {
                if col == '#' {
                    active.insert((i as i32, j as i32, 0, 0));
                }
            }
        }
        Self { active }
    }
    fn part_1(&self) -> usize {
        let mut neighbors = Vec::new();
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    if !(x == 0 && y == 0 && z == 0) {
                        neighbors.push((x, y, z, 0));
                    }
                }
            }
        }
        self.simulate(&neighbors)
    }
    fn part_2(&self) -> usize {
        let mut neighbors = Vec::new();
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    for w in -1..=1 {
                        if !(x == 0 && y == 0 && z == 0 && w == 0) {
                            neighbors.push((x, y, z, w));
                        }
                    }
                }
            }
        }
        self.simulate(&neighbors)
    }
    fn simulate(&self, neighbors: &[(i32, i32, i32, i32)]) -> usize {
        let mut active = self.active.clone();
        for _ in 0..6 {
            let mut targets = HashSet::new();
            for &p in &active {
                targets.insert(p);
                for &d in neighbors.iter() {
                    targets.insert((p.0 + d.0, p.1 + d.1, p.2 + d.2, p.3 + d.3));
                }
            }
            active = targets
                .into_iter()
                .filter(|&p| {
                    let count = neighbors
                        .iter()
                        .filter(|&d| active.contains(&(p.0 + d.0, p.1 + d.1, p.2 + d.2, p.3 + d.3)))
                        .count();
                    count == 3 || (count == 2 && active.contains(&p))
                })
                .collect();
        }
        active.len()
    }
}

fn main() {
    let solution = Solution::new(
        &BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(Result::ok)
            .collect::<Vec<_>>(),
    );
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_inputs() -> Vec<String> {
        r"
.#.
..#
###"
        .split('\n')
        .skip(1)
        .map(str::to_string)
        .collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(112, Solution::new(&example_inputs()).part_1());
    }

    #[test]
    fn example_2() {
        assert_eq!(848, Solution::new(&example_inputs()).part_2());
    }
}
