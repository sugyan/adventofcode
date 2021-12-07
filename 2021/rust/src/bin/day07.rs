use std::io::{BufRead, BufReader};

struct Solution {
    positions: Vec<i32>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        Self {
            positions: inputs[0].split(',').map(|x| x.parse().unwrap()).collect(),
        }
    }
    fn part_1(&self) -> i32 {
        self.total_fuels(true)
    }
    fn part_2(&self) -> i32 {
        self.total_fuels(false)
    }
    fn total_fuels(&self, constant: bool) -> i32 {
        let min = *self.positions.iter().min().unwrap();
        let max = *self.positions.iter().max().unwrap();
        (min..=max)
            .map(|i| {
                self.positions
                    .iter()
                    .map(|p| {
                        let d = (p - i).abs();
                        match constant {
                            true => d,
                            false => d * (d + 1) / 2,
                        }
                    })
                    .sum()
            })
            .min()
            .unwrap()
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
        vec![String::from("16,1,2,0,4,2,7,1,2,14")]
    }

    #[test]
    fn example_1() {
        assert_eq!(37, Solution::new(&example_inputs()).part_1());
    }

    #[test]
    fn example_2() {
        assert_eq!(168, Solution::new(&example_inputs()).part_2());
    }
}
