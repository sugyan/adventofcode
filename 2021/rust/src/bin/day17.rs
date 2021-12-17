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
        let y = -(self.target_area.1).0 - 1;
        y * (y + 1) / 2
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
}
