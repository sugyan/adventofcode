use itertools::Itertools;
use std::io::{BufRead, BufReader};

struct Solution {
    target_area: ((i32, i32), (i32, i32)),
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        Self {
            target_area: inputs[0][13..]
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
    fn part_1(&self) -> i32 {
        let y = -self.target_area.1 .0 - 1;
        y * (y + 1) / 2
    }
    fn part_2(&self) -> usize {
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
                    vx -= if vx > 0 { 1 } else { 0 };
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
    let solution = Solution::new(
        &BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(Result::ok)
            .collect::<Vec<_>>(),
    );
    println!("{}", solution.part_1());
    println!("{}", solution.part_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            45,
            Solution::new(&[String::from("target area: x=20..30, y=-10..-5")]).part_1()
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            112,
            Solution::new(&[String::from("target area: x=20..30, y=-10..-5")]).part_2()
        );
    }
}
