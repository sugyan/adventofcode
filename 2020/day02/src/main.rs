use regex::Regex;
use std::io::{BufRead, BufReader};

struct Solution {
    inputs: Vec<String>,
    re: Regex,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        Self {
            inputs,
            re: Regex::new(r"^(\d+)\-(\d+) (.): (.+)$").unwrap(),
        }
    }
    fn solve_1(&self) -> usize {
        self.inputs
            .iter()
            .filter(|&input| {
                if let Some(cap) = self.re.captures(input) {
                    let min = cap[1].parse::<usize>().unwrap();
                    let max = cap[2].parse::<usize>().unwrap();
                    let chr = cap[3].chars().next().unwrap();
                    let appear = cap[4].chars().filter(|&c| c == chr).count();
                    (min..=max).contains(&appear)
                } else {
                    false
                }
            })
            .count()
    }
    fn solve_2(&self) -> usize {
        self.inputs
            .iter()
            .filter(|&input| {
                if let Some(cap) = self.re.captures(input) {
                    let pos1 = cap[1].parse::<usize>().unwrap();
                    let pos2 = cap[2].parse::<usize>().unwrap();
                    let chr = cap[3].chars().next().unwrap();
                    matches!(
                        (
                            cap[4].chars().nth(pos1 - 1) == Some(chr),
                            cap[4].chars().nth(pos2 - 1) == Some(chr),
                        ),
                        (true, false) | (false, true),
                    )
                } else {
                    false
                }
            })
            .count()
    }
}

fn main() {
    let inputs: Vec<String> = BufReader::new(std::io::stdin().lock())
        .lines()
        .filter_map(|line| line.ok())
        .collect();
    let solution = Solution::new(inputs);
    println!("{}", solution.solve_1());
    println!("{}", solution.solve_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_inputs() -> Vec<String> {
        r"
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"
            .split('\n')
            .skip(1)
            .map(str::to_string)
            .collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(2, Solution::new(example_inputs()).solve_1())
    }

    #[test]
    fn example_2() {
        assert_eq!(1, Solution::new(example_inputs()).solve_2())
    }
}
