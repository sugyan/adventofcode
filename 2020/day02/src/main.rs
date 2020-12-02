use regex::Regex;
use std::io::{BufRead, BufReader};

struct Solution {
    inputs: Vec<String>,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        Self { inputs }
    }
    fn solve_1(&self) -> usize {
        let mut ret = 0;
        if let Ok(re) = Regex::new(r"^(\d+)\-(\d+) (.): (.+)$") {
            for input in self.inputs.iter() {
                for cap in re.captures_iter(input) {
                    let min = cap[1].parse::<usize>().unwrap();
                    let max = cap[2].parse::<usize>().unwrap();
                    let chr = cap[3].chars().next().unwrap();
                    let appear = cap[4].chars().filter(|&c| c == chr).count();
                    if min <= appear && appear <= max {
                        ret += 1;
                    }
                }
            }
        }
        ret
    }
}

fn main() {
    let inputs: Vec<String> = BufReader::new(std::io::stdin().lock())
        .lines()
        .filter_map(|line| line.ok())
        .collect();
    let solution = Solution::new(inputs);
    println!("{}", solution.solve_1());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            2,
            Solution::new(vec![
                String::from("1-3 a: abcde"),
                String::from("1-3 b: cdefg"),
                String::from("2-9 c: ccccccccc")
            ])
            .solve_1()
        )
    }
}
