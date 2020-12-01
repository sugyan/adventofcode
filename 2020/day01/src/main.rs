use std::collections::HashSet;
use std::io::{BufRead, BufReader};

struct Solution {
    inputs: Vec<i32>,
}

impl Solution {
    fn new(inputs: Vec<i32>) -> Self {
        Self { inputs }
    }
    fn solve_1(&self) -> i32 {
        let hs: HashSet<&i32> = self.inputs.iter().collect();
        for &i in self.inputs.iter() {
            if hs.contains(&(2020 - i)) {
                return (2020 - i) * i;
            }
        }
        0
    }
    fn solve_2(&self) -> i32 {
        let hs: HashSet<&i32> = self.inputs.iter().collect();
        for i in 0..self.inputs.len() - 1 {
            for j in i + 1..self.inputs.len() {
                if hs.contains(&(2020 - self.inputs[i] - self.inputs[j])) {
                    return (2020 - self.inputs[i] - self.inputs[j])
                        * self.inputs[i]
                        * self.inputs[j];
                }
            }
        }
        0
    }
}

fn main() {
    let inputs: Vec<i32> = BufReader::new(std::io::stdin().lock())
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|s| s.parse().ok())
        .collect();
    let solution = Solution::new(inputs);
    println!("{}", solution.solve_1());
    println!("{}", solution.solve_2());
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            514579,
            Solution::new(vec![1721, 979, 366, 299, 675, 1456]).solve_1()
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            241861950,
            Solution::new(vec![1721, 979, 366, 299, 675, 1456]).solve_2()
        );
    }
}
