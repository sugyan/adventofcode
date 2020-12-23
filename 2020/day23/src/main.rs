use std::collections::HashMap;
use std::io::{BufRead, BufReader};

struct Solution {
    cups: Vec<u64>,
}

impl Solution {
    fn new(input: String) -> Self {
        Self {
            cups: input
                .as_bytes()
                .iter()
                .map(|&b| (b - b'0') as u64)
                .collect(),
        }
    }
    fn solve_1(&self) -> String {
        let cups = self.game(9, 100);
        let mut ret = Vec::new();
        let mut cup = 1;
        for _ in 0..8 {
            if let Some(&next) = cups.get(&cup) {
                ret.push(next.to_string());
                cup = next;
            }
        }
        ret.concat()
    }
    fn solve_2(&self) -> u64 {
        let cups = self.game(1_000_000, 10_000_000);
        let mut ret = 1;
        let mut cup = 1;
        for _ in 0..2 {
            if let Some(&next) = cups.get(&cup) {
                ret *= next;
                cup = next;
            }
        }
        ret
    }
    fn game(&self, cups: usize, moves: usize) -> HashMap<u64, u64> {
        let mut hm = HashMap::with_capacity(cups);
        let mut last = None;
        for (i, &cup) in self.cups.iter().enumerate() {
            if i > 0 {
                hm.insert(self.cups[i - 1], cup);
            }
            last = Some(cup);
        }
        for i in self.cups.len()..cups {
            if let Some(l) = last {
                hm.insert(l, i as u64 + 1);
            }
            last = Some(i as u64 + 1)
        }
        if let Some(l) = last {
            hm.insert(l, self.cups[0]);
        }

        let highest = cups as u64;
        let mut current = self.cups[0];
        let mut pickups = [0; 3];
        for _ in 0..moves {
            let mut p = current;
            for pickup in pickups.iter_mut() {
                if let Some(&next) = hm.get(&p) {
                    p = next;
                }
                *pickup = p;
            }
            let mut destination = if current > 1 { current - 1 } else { highest };
            while pickups.iter().any(|&x| x == destination) {
                destination = if destination > 1 {
                    destination - 1
                } else {
                    highest
                };
            }
            if let (Some(&c_next), Some(&p_next), Some(&d_next)) =
                (hm.get(&current), hm.get(&p), hm.get(&destination))
            {
                hm.insert(current, p_next);
                hm.insert(p, d_next);
                hm.insert(destination, c_next);
            }
            if let Some(&next) = hm.get(&current) {
                current = next;
            }
        }
        hm
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
    println!("{}", solution.solve_2());
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

    #[test]
    fn example_2() {
        assert_eq!(
            149245887792,
            Solution::new(String::from("389125467")).solve_2()
        );
    }
}
