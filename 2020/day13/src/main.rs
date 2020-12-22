use std::io::{BufRead, BufReader};

struct Solution {
    inputs: Vec<String>,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        Self { inputs }
    }
    fn solve_1(&self) -> i32 {
        let timestamp = self.inputs[0].parse::<i32>().unwrap();
        if let Some((minutes, id)) = self.inputs[1]
            .split(',')
            .filter_map(|s| s.parse::<i32>().ok())
            .map(|id| (id * (timestamp / id + 1) - timestamp, id))
            .min_by_key(|&e| e.0)
        {
            id * minutes
        } else {
            0
        }
    }
    fn solve_2(&self) -> i64 {
        self.inputs[1]
            .split(',')
            .enumerate()
            .filter_map(|(i, s)| {
                if let Ok(id) = s.parse::<i64>() {
                    Some((i as i64, id))
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
        BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(|line| line.ok())
            .collect(),
    );
    println!("{}", solution.solve_1());
    println!("{}", solution.solve_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            295,
            Solution::new(
                "
939
7,13,x,x,59,x,31,19"[1..]
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect()
            )
            .solve_1()
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            1068781,
            Solution::new(vec![String::new(), String::from("7,13,x,x,59,x,31,19")]).solve_2()
        );
        assert_eq!(
            3417,
            Solution::new(vec![String::new(), String::from("17,x,13,19")]).solve_2()
        );
        assert_eq!(
            754018,
            Solution::new(vec![String::new(), String::from("67,7,59,61")]).solve_2()
        );
        assert_eq!(
            779210,
            Solution::new(vec![String::new(), String::from("67,x,7,59,61")]).solve_2()
        );
        assert_eq!(
            1261476,
            Solution::new(vec![String::new(), String::from("67,7,x,59,61")]).solve_2()
        );
        assert_eq!(
            1202161486,
            Solution::new(vec![String::new(), String::from("1789,37,47,1889")]).solve_2()
        );
    }
}
