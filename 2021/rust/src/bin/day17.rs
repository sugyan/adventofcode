use aoc2021::Solve;
use itertools::Itertools;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    target_area: ((i32, i32), (i32, i32)),
}

impl Solve for Solution {
    type Answer1 = i32;
    type Answer2 = usize;

    fn new(r: impl Read) -> Self {
        Self {
            target_area: BufReader::new(r).lines().find_map(Result::ok).unwrap()[13..]
                .split(", ")
                .map(|s| {
                    s[2..]
                        .split("..")
                        .map(|v| v.parse().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let y = -self.target_area.1 .0 - 1;
        y * (y + 1) / 2
    }
    fn part2(&self) -> Self::Answer2 {
        let tmax = 2 * (-self.target_area.1 .0 - 1) as usize + 1;
        let mut ys = vec![Vec::new(); tmax + 1];
        for ivy in self.target_area.1 .0..-self.target_area.1 .0 {
            let mut y = 0;
            let mut vy = ivy;
            for v in ys.iter_mut() {
                y += vy;
                vy -= 1;
                if (self.target_area.1 .0..=self.target_area.1 .1).contains(&y) {
                    v.push(ivy);
                }
            }
        }
        (0..=self.target_area.0 .1)
            .map(|ivx| {
                let mut ivys = Vec::<&i32>::new();
                let mut x = 0;
                let mut vx = ivx;
                for v in ys.iter_mut() {
                    x += vx;
                    vx -= i32::from(vx > 0);
                    if (self.target_area.0 .0..=self.target_area.0 .1).contains(&x) {
                        ivys.extend(v.iter());
                    }
                }
                ivys.sort();
                ivys.dedup();
                ivys.len()
            })
            .sum()
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
target area: x=20..30, y=-10..-5
"[1..]
            .as_bytes()
    }

    #[test]
    fn example1() {
        assert_eq!(45, Solution::new(example_input()).part1());
    }

    #[test]
    fn example2() {
        assert_eq!(112, Solution::new(example_input()).part2());
    }
}
