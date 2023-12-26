use aoc2023::Solve;
use itertools::Itertools;
use std::io::{BufRead, BufReader, Read};

type Position = (i64, i64, i64);
type Velocity = (i64, i64, i64);

struct Solution {
    hailstones: Vec<(Position, Velocity)>,
}

impl Solution {
    fn count_intersections(&self, test_area: (i64, i64)) -> usize {
        let min = test_area.0 as f64;
        let max = test_area.1 as f64;
        self.hailstones
            .iter()
            .combinations(2)
            .filter(|c| {
                let &((px0, py0, _), (vx0, vy0, _)) = c[0];
                let &((px1, py1, _), (vx1, vy1, _)) = c[1];
                let a0 = vy0 as f64 / vx0 as f64;
                let a1 = vy1 as f64 / vx1 as f64;
                let b0 = (py0 * vx0 - px0 * vy0) as f64 / (vx0 as f64);
                let b1 = (py1 * vx1 - px1 * vy1) as f64 / (vx1 as f64);
                let x = (b1 - b0) / (a0 - a1);
                let y = a0 * x + b0;
                (min <= x && x <= max && min <= y && y <= max)
                    && ((x - px0 as f64).signum() == (vx0 as f64).signum())
                    && ((x - px1 as f64).signum() == (vx1 as f64).signum())
            })
            .count()
    }
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        Self {
            hailstones: BufReader::new(r)
                .lines()
                .map_while(Result::ok)
                .map(|line| {
                    let (pos, vel) = line.split_once(" @ ").expect("should be valid line");
                    let parse = |s: &str| {
                        s.split(", ")
                            .map(|s| s.trim().parse().expect("should be valid number"))
                            .collect_tuple()
                            .expect("should be valid tuple")
                    };
                    (parse(pos), parse(vel))
                })
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.count_intersections((200_000_000_000_000, 400_000_000_000_000))
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

    fn example_input() -> &'static [u8] {
        r"
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(
            Solution::new(example_input()).count_intersections((7, 27)),
            2
        );
    }
}
