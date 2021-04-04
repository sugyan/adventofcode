use std::io::{BufRead, BufReader};

struct Solution {
    modules: Vec<i32>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        Self {
            modules: inputs.iter().filter_map(|s| s.parse().ok()).collect(),
        }
    }
    fn part_1(&self) -> i32 {
        self.modules.iter().map(|&mass| mass / 3 - 2).sum()
    }
    fn part_2(&self) -> i32 {
        self.modules
            .iter()
            .map(|&mass| {
                let (mut m, mut ret) = (mass, 0);
                while m > 0 {
                    m = (m / 3 - 2).max(0);
                    ret += m;
                }
                ret
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
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(2, Solution::new(&[String::from("12")]).part_1());
        assert_eq!(2, Solution::new(&[String::from("14")]).part_1());
        assert_eq!(654, Solution::new(&[String::from("1969")]).part_1());
        assert_eq!(33583, Solution::new(&[String::from("100756")]).part_1());
    }

    #[test]
    fn example_2() {
        assert_eq!(2, Solution::new(&[String::from("14")]).part_2());
        assert_eq!(966, Solution::new(&[String::from("1969")]).part_2());
        assert_eq!(50346, Solution::new(&[String::from("100756")]).part_2());
    }
}
