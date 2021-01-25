use std::io::{BufRead, BufReader};

struct Solution {
    cups: Vec<u32>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        Self {
            cups: inputs[0]
                .as_bytes()
                .iter()
                .map(|&b| u32::from(b - b'0'))
                .collect(),
        }
    }
    fn part_1(&self) -> String {
        let cups = self.game(9, 100);
        let mut cup = 1;
        (0..8)
            .map(|_| {
                cup = cups[cup] as usize;
                cup.to_string()
            })
            .collect()
    }
    fn part_2(&self) -> u64 {
        let cups = self.game(1_000_000, 10_000_000);
        let mut cup = 1;
        (0..2)
            .map(|_| {
                cup = cups[cup] as usize;
                cup as u64
            })
            .product()
    }
    fn game(&self, cups: usize, moves: usize) -> Vec<u32> {
        let mut v = vec![0; cups + 1];
        let mut last = None;
        for (i, &cup) in self.cups.iter().enumerate() {
            if i > 0 {
                v[self.cups[i - 1] as usize] = cup;
            }
            last = Some(cup);
        }
        for i in self.cups.len()..cups {
            if let Some(l) = last {
                v[l as usize] = i as u32 + 1;
            }
            last = Some(i as u32 + 1)
        }
        if let Some(l) = last {
            v[l as usize] = self.cups[0];
        }

        let highest = cups as usize;
        let mut current = self.cups[0] as usize;
        let mut pickups = [0; 3];
        for _ in 0..moves {
            let mut p = current;
            for pickup in &mut pickups {
                p = v[p] as usize;
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
            let tmp = v[current];
            v[current] = v[p];
            v[p] = v[destination];
            v[destination] = tmp;
            current = v[current] as usize;
        }
        v
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
    fn example_1() {
        assert_eq!(
            "67384529",
            Solution::new(&[String::from("389125467")]).part_1()
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            149_245_887_792,
            Solution::new(&[String::from("389125467")]).part_2()
        );
    }
}
