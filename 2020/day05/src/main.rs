use std::io::{BufRead, BufReader};

struct Solution {
    inputs: Vec<String>,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        Self { inputs }
    }
    fn solve_1(&self) -> i32 {
        self.inputs
            .iter()
            .map(|seat| {
                seat.chars()
                    .rev()
                    .enumerate()
                    .map(|(i, c)| if matches!(c, 'B' | 'R') { 1 << i } else { 0 })
                    .sum()
            })
            .max()
            .unwrap()
    }
}

fn main() {
    let solution = Solution::new(
        BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(|line| line.ok())
            .collect(),
    );
    println!("{}", solution.solve_1());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            820,
            Solution::new(vec![
                String::from("BFFFBBFRRR"),
                String::from("FFFBBBFRRR"),
                String::from("BBFFBBFRLL"),
            ])
            .solve_1()
        );
    }
}
