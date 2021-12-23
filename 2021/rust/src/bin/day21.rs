use std::io::{BufRead, BufReader};

struct Solution {
    starting_positions: Vec<u8>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        Self {
            starting_positions: inputs
                .iter()
                .map(|s| s.bytes().last().unwrap() - b'0')
                .collect(),
        }
    }
    fn part_1(&self) -> u32 {
        let mut positions = self.starting_positions.clone();
        let mut scores = vec![0; positions.len()];
        let mut i = 0;
        loop {
            for (j, score) in scores.iter_mut().enumerate() {
                positions[j] = (positions[j] + ((i * 3 + 6) % 10) as u8) % 10;
                *score += ((positions[j] + 9) % 10) as u32 + 1;
                i += 3;
                if *score >= 1000 {
                    return scores[1 - j] * i;
                }
            }
        }
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

    fn example_inputs() -> Vec<String> {
        r"
Player 1 starting position: 4
Player 2 starting position: 8"[1..]
            .split('\n')
            .map(String::from)
            .collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(739785, Solution::new(&example_inputs()).part_1());
    }
}
