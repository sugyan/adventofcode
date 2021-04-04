use std::io::{BufRead, BufReader};

struct Solution {
    range: (u32, u32),
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        let v = inputs[0]
            .split('-')
            .map(str::parse)
            .filter_map(Result::ok)
            .collect::<Vec<_>>();
        Self {
            range: (v[0], v[1]),
        }
    }
    fn part_1(&self) -> usize {
        (self.range.0..=self.range.1)
            .filter(|&n| Self::check_1(n))
            .count()
    }
    fn part_2(&self) -> usize {
        (self.range.0..=self.range.1)
            .filter(|&n| Self::check_2(n))
            .count()
    }
    fn check_1(password: u32) -> bool {
        let v = password.to_string().bytes().collect::<Vec<_>>();
        v.windows(2).all(|b| b[0] <= b[1]) && v.windows(2).any(|b| b[0] == b[1])
    }
    fn check_2(password: u32) -> bool {
        let v = password.to_string().bytes().collect::<Vec<_>>();
        v.windows(2).all(|b| b[0] <= b[1])
            && v.iter()
                .fold([0; 10], |mut acc, &b| {
                    acc[(b - b'0') as usize] += 1;
                    acc
                })
                .iter()
                .any(|&c| c == 2)
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
    fn check_1() {
        assert_eq!(true, Solution::check_1(111_111));
        assert_eq!(false, Solution::check_1(223_450));
        assert_eq!(false, Solution::check_1(123_789));
    }

    #[test]
    fn check_2() {
        assert_eq!(true, Solution::check_2(112_233));
        assert_eq!(false, Solution::check_2(123_444));
        assert_eq!(true, Solution::check_2(111_122));
    }
}
