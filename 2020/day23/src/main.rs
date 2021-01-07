use std::io::{BufRead, BufReader};

struct Solution {
    cups: Vec<u32>,
}

impl Solution {
    fn new(input: String) -> Self {
        Self {
            cups: input
                .as_bytes()
                .iter()
                .map(|&b| (b - b'0') as u32)
                .collect(),
        }
    }
    fn solve_1(&self) -> String {
        let cups = self.game(9, 100);
        let mut ret = Vec::new();
        let mut cup = 1;
        for _ in 0..8 {
            ret.push(cups[cup].to_string());
            cup = cups[cup] as usize;
        }
        ret.concat()
    }
    fn solve_2(&self) -> u64 {
        let cups = self.game(1_000_000, 10_000_000);
        let mut ret = 1;
        let mut cup = 1;
        for _ in 0..2 {
            ret *= cups[cup] as u64;
            cup = cups[cup] as usize;
        }
        ret
    }
    fn game(&self, cups: usize, moves: usize) -> Vec<u32> {
        let mut map = vec![0; cups + 1];
        let mut last = None;
        for (i, &cup) in self.cups.iter().enumerate() {
            if i > 0 {
                map[self.cups[i - 1] as usize] = cup;
            }
            last = Some(cup);
        }
        for i in self.cups.len()..cups {
            if let Some(l) = last {
                map[l as usize] = i as u32 + 1;
            }
            last = Some(i as u32 + 1)
        }
        if let Some(l) = last {
            map[l as usize] = self.cups[0];
        }

        let highest = cups as usize;
        let mut current = self.cups[0] as usize;
        let mut pickups = [0; 3];
        for _ in 0..moves {
            let mut p = current;
            for pickup in pickups.iter_mut() {
                p = map[p] as usize;
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
            let tmp = map[current];
            map[current] = map[p];
            map[p] = map[destination];
            map[destination] = tmp;
            current = map[current] as usize;
        }
        map
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
