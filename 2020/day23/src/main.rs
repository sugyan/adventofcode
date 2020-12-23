use std::io::{BufRead, BufReader};

struct Solution {
    cups: Vec<u8>,
}

impl Solution {
    fn new(input: String) -> Self {
        Self {
            cups: input.as_bytes().iter().map(|&b| b - b'0').collect(),
        }
    }
    fn solve_1(&self) -> String {
        let mut cups = self.cups.clone();
        for _ in 0..100 {
            let current = cups.remove(0);
            let pickup: Vec<u8> = (0..3).map(|_| cups.remove(0)).collect();
            let mut destination = if current == 1 { 9 } else { current - 1 };
            while pickup.contains(&destination) {
                destination = if destination == 1 { 9 } else { destination - 1 };
            }
            if let Some(p) = cups.iter().position(|&x| x == destination) {
                cups.splice(p + 1..p + 1, pickup);
            }
            cups.push(current);
        }
        while cups[0] != 1 {
            let first = cups.remove(0);
            cups.push(first);
        }
        cups.iter().skip(1).map(|&u| (u + b'0') as char).collect()
    }
}

fn main() {
    let solution = Solution::new(
        BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(|line| line.ok())
            .next()
            .unwrap(),
    );
    println!("{}", solution.solve_1());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            "67384529",
            Solution::new(String::from("389125467")).solve_1()
        );
    }
}
