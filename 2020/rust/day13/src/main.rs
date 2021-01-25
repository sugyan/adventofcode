use std::io::{BufRead, BufReader};

struct Solution {
    inputs: Vec<String>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        Self {
            inputs: inputs.iter().map(String::to_string).collect(),
        }
    }
    fn part_1(&self) -> u32 {
        let timestamp = self.inputs[0].parse::<u32>().unwrap();
        if let Some((minutes, id)) = self.inputs[1]
            .split(',')
            .filter_map(|s| {
                s.parse::<u32>()
                    .ok()
                    .map(|id| (id * (timestamp / id + 1) - timestamp, id))
            })
            .min_by_key(|&e| e.0)
        {
            id * minutes
        } else {
            0
        }
    }
    fn part_2(&self) -> u64 {
        self.inputs[1]
            .split(',')
            .enumerate()
            .filter_map(|(i, s)| {
                if let Ok(id) = s.parse::<u64>() {
                    Some((i as u64, id))
                } else {
                    None
                }
            })
            .fold((1, 0), |(a, b), (i, id)| {
                let m = (0..).find(|m| (a * m + b + i) % id == 0).unwrap();
                (a * id, a * m + b)
            })
            .1
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

    fn example_inputs() -> Vec<String> {
        r"
939
7,13,x,x,59,x,31,19"
            .split('\n')
            .skip(1)
            .map(str::to_string)
            .collect()
    }
    #[test]
    fn example_1() {
        assert_eq!(295, Solution::new(&example_inputs()).part_1());
    }

    #[test]
    fn example_2() {
        assert_eq!(1_068_781, Solution::new(&example_inputs()).part_2());
        assert_eq!(
            3417,
            Solution::new(&[String::new(), String::from("17,x,13,19")]).part_2()
        );
        assert_eq!(
            754_018,
            Solution::new(&[String::new(), String::from("67,7,59,61")]).part_2()
        );
        assert_eq!(
            779_210,
            Solution::new(&[String::new(), String::from("67,x,7,59,61")]).part_2()
        );
        assert_eq!(
            1_261_476,
            Solution::new(&[String::new(), String::from("67,7,x,59,61")]).part_2()
        );
        assert_eq!(
            1_202_161_486,
            Solution::new(&[String::new(), String::from("1789,37,47,1889")]).part_2()
        );
    }
}
