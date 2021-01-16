use std::io::{BufRead, BufReader};

struct Solution {
    seats: Vec<i32>,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        Self {
            seats: inputs
                .iter()
                .map(|seat| {
                    seat.chars()
                        .rev()
                        .enumerate()
                        .map(|(i, c)| if matches!(c, 'B' | 'R') { 1 << i } else { 0 })
                        .sum()
                })
                .collect(),
        }
    }
    fn part_1(&self) -> i32 {
        *self.seats.iter().max().unwrap()
    }
    fn part_2(&self) -> i32 {
        let offset = self.part_1() as usize - self.seats.len();
        let mut v = vec![false; self.seats.len() + 1];
        for &seat in self.seats.iter() {
            v[seat as usize - offset] = true;
        }
        (v.iter().position(|&b| !b).unwrap() + offset) as i32
    }
}

fn main() {
    let solution = Solution::new(
        BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(|line| line.ok())
            .collect(),
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
            820,
            Solution::new(vec![
                String::from("BFFFBBFRRR"),
                String::from("FFFBBBFRRR"),
                String::from("BBFFBBFRLL"),
            ])
            .part_1()
        );
    }
}
