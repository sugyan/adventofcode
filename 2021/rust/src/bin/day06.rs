use std::io::{BufRead, BufReader};

struct Solution {
    timers: Vec<u8>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        Self {
            timers: inputs[0]
                .split(',')
                .map(|x| x.parse::<u8>().unwrap())
                .collect(),
        }
    }
    fn part_1(&self) -> usize {
        let mut timers = self.timers.clone();
        for _ in 0..80 {
            for i in 0..timers.len() {
                if timers[i] == 0 {
                    timers.push(8);
                    timers[i] = 6
                } else {
                    timers[i] -= 1;
                };
            }
        }
        timers.len()
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
        assert_eq!(5934, Solution::new(&[String::from("3,4,3,1,2")]).part_1());
    }
}
