use std::io::{BufRead, BufReader};

struct Solution {
    inputs: Vec<usize>,
}

impl Solution {
    fn new(input: String) -> Self {
        Self {
            inputs: input.split(',').filter_map(|s| s.parse().ok()).collect(),
        }
    }
    fn solve_1(&self) -> u32 {
        self.play(2020)
    }
    fn solve_2(&self) -> u32 {
        self.play(30_000_000)
    }
    fn play(&self, num: u32) -> u32 {
        let mut memory: Vec<u32> = vec![0; num as usize];
        self.inputs
            .iter()
            .enumerate()
            .for_each(|(i, &input)| memory[input] = i as u32 + 1);
        let mut prev = self.inputs[self.inputs.len() - 1] as u32;
        for i in self.inputs.len() as u32..num {
            let last_index = memory[prev as usize];
            memory[prev as usize] = i;
            prev = if last_index == 0 { 0 } else { i - last_index };
        }
        prev
    }
}

fn main() {
    let solution = Solution::new(
        BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(|line| line.ok())
            .next()
            .unwrap(),
    );
    println!("{}", solution.solve_1());
    println!("{}", solution.solve_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(436, Solution::new(String::from("0,3,6")).solve_1());
        assert_eq!(1, Solution::new(String::from("1,3,2")).solve_1());
        assert_eq!(10, Solution::new(String::from("2,1,3")).solve_1());
        assert_eq!(27, Solution::new(String::from("1,2,3")).solve_1());
        assert_eq!(78, Solution::new(String::from("2,3,1")).solve_1());
        assert_eq!(438, Solution::new(String::from("3,2,1")).solve_1());
        assert_eq!(1836, Solution::new(String::from("3,1,2")).solve_1());
    }

    #[test]
    fn example_2() {
        assert_eq!(175594, Solution::new(String::from("0,3,6")).solve_2());
        // assert_eq!(2578, Solution::new(String::from("1,3,2")).solve_2());
        // assert_eq!(3544142, Solution::new(String::from("2,1,3")).solve_2());
        // assert_eq!(261214, Solution::new(String::from("1,2,3")).solve_2());
        // assert_eq!(6895259, Solution::new(String::from("2,3,1")).solve_2());
        // assert_eq!(18, Solution::new(String::from("3,2,1")).solve_2());
        // assert_eq!(362, Solution::new(String::from("3,1,2")).solve_2());
    }
}
