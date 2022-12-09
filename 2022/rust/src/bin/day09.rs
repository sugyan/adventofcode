use aoc2022::Solve;
use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    motions: Vec<((i32, i32), u32)>,
}

impl Solution {
    fn tail_visited(&self, knots_count: usize) -> usize {
        let mut knots = vec![(0, 0); knots_count];
        let mut hs = HashSet::new();
        for &(d, n) in &self.motions {
            for _ in 0..n {
                knots[0].0 += d.0;
                knots[0].1 += d.1;
                for i in 1..knots_count {
                    let diff = (knots[i - 1].0 - knots[i].0, knots[i - 1].1 - knots[i].1);
                    if diff.0.abs() > 1 || diff.1.abs() > 1 {
                        knots[i].0 += diff.0.signum();
                        knots[i].1 += diff.1.signum();
                    }
                }
                hs.insert(knots[knots_count - 1]);
            }
        }
        hs.len()
    }
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;

    fn new(r: impl Read) -> Self {
        Self {
            motions: BufReader::new(r)
                .lines()
                .filter_map(Result::ok)
                .map(|s| {
                    (
                        match &s[..1] {
                            "R" => (1, 0),
                            "U" => (0, 1),
                            "L" => (-1, 0),
                            "D" => (0, -1),
                            _ => unreachable!(),
                        },
                        s[2..].parse().unwrap(),
                    )
                })
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.tail_visited(2)
    }
    fn part2(&self) -> Self::Answer2 {
        self.tail_visited(10)
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
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"[1..]
            .as_bytes()
    }

    fn example_input_large() -> &'static [u8] {
        r"
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(13, Solution::new(example_input()).part1());
    }

    #[test]
    fn part2() {
        assert_eq!(1, Solution::new(example_input()).part2());
        assert_eq!(36, Solution::new(example_input_large()).part2());
    }
}
