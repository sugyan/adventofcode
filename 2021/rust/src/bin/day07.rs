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
        let max = *self.positions.iter().max().unwrap();
        let mut v = vec![0; max as usize + 1];
        for &pos in &self.positions {
            v[pos as usize] += 1;
        }
        let mut sum = self.positions.iter().sum::<i32>();
        let mut ret = sum;
        let mut num = 0;
        for &n in v.iter() {
            num += n;
            sum += num * 2 - self.positions.len() as i32;
            ret = ret.min(sum);
        }
        ret
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
        vec![String::from("16,1,2,0,4,2,7,1,2,14")]
    }

    #[test]
    fn example_1() {
        assert_eq!(37, Solution::new(&example_inputs()).part_1());
    }
}
