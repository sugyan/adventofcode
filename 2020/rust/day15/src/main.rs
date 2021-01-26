use std::io::{BufRead, BufReader};

struct Solution {
    numbers: Vec<u32>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        Self {
            numbers: inputs[0]
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect(),
        }
    }
    fn part_1(&self) -> u32 {
        self.play(2020)
    }
    fn part_2(&self) -> u32 {
        self.play(30_000_000)
    }
    fn play(&self, num: u32) -> u32 {
        let mut memory = vec![0; num as usize];
        self.numbers
            .iter()
            .enumerate()
            .for_each(|(i, &number)| memory[number as usize] = i as u32 + 1);
        let mut prev = self.numbers[self.numbers.len() - 1];
        for i in self.numbers.len() as u32..num {
            let last_index = memory[prev as usize];
            memory[prev as usize] = i;
            prev = if last_index == 0 { 0 } else { i - last_index };
        }
        prev
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
        assert_eq!(436, Solution::new(&[String::from("0,3,6")]).part_1());
        assert_eq!(1, Solution::new(&[String::from("1,3,2")]).part_1());
        assert_eq!(10, Solution::new(&[String::from("2,1,3")]).part_1());
        assert_eq!(27, Solution::new(&[String::from("1,2,3")]).part_1());
        assert_eq!(78, Solution::new(&[String::from("2,3,1")]).part_1());
        assert_eq!(438, Solution::new(&[String::from("3,2,1")]).part_1());
        assert_eq!(1836, Solution::new(&[String::from("3,1,2")]).part_1());
    }

    #[test]
    fn example_2() {
        assert_eq!(175_594, Solution::new(&[String::from("0,3,6")]).part_2());
        // assert_eq!(2578, Solution::new(&[String::from("1,3,2")]).part_2());
        // assert_eq!(3_544_142, Solution::new(&[String::from("2,1,3")]).part_2());
        // assert_eq!(261_214, Solution::new(&[String::from("1,2,3")]).part_2());
        // assert_eq!(6_895_259, Solution::new(&[String::from("2,3,1")]).part_2());
        // assert_eq!(18, Solution::new(&[String::from("3,2,1")]).part_2());
        // assert_eq!(362, Solution::new(&[String::from("3,1,2")]).part_2());
    }
}
