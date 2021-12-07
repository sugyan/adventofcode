use std::io::{BufRead, BufReader};

struct Solution {
    timers: Vec<u8>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        Self {
            timers: inputs[0].split(',').map(|x| x.parse().unwrap()).collect(),
        }
    }
    fn part_1(&self) -> u64 {
        self.count_lanternfishes(80)
    }
    fn part_2(&self) -> u64 {
        self.count_lanternfishes(256)
    }
    fn count_lanternfishes(&self, days: usize) -> u64 {
        let mut counts = [0; 9];
        for &t in &self.timers {
            counts[t as usize] += 1;
        }
        for _ in 0..days {
            counts.rotate_left(1);
            counts[6] += counts[8];
        }
        counts.iter().sum()
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

    fn example_inputs() -> Vec<String> {
        vec![String::from("3,4,3,1,2")]
    }

    #[test]
    fn example_1() {
        assert_eq!(5934, Solution::new(&example_inputs()).part_1());
    }

    #[test]
    fn example_2() {
        assert_eq!(26_984_457_539, Solution::new(&example_inputs()).part_2());
    }
}
