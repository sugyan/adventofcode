use aoc2022::Solve;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    numbers: Vec<i32>,
}

impl Solve for Solution {
    type Answer1 = i32;
    type Answer2 = i32;

    fn new(r: impl Read) -> Self {
        Self {
            numbers: BufReader::new(r)
                .lines()
                .filter_map(Result::ok)
                .filter_map(|s| s.parse().ok())
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let mut indices = (0..self.numbers.len())
            .map(|i| {
                (
                    (i + self.numbers.len() - 1) % self.numbers.len(),
                    (i + 1) % self.numbers.len(),
                )
            })
            .collect::<Vec<_>>();
        for i in 0..self.numbers.len() {
            let mut j = i;
            match self.numbers[i].rem_euclid(self.numbers.len() as i32 - 1) {
                0 => continue,
                num => (0..num).for_each(|_| j = indices[j].1),
            }
            let (p, n) = indices[i];
            indices[p].1 = n;
            indices[n].0 = p;
            let k = indices[j].1;
            indices[j].1 = i;
            indices[k].0 = i;
            indices[i].0 = j;
            indices[i].1 = k;
        }
        let mut i = 0;
        while self.numbers[i] != 0 {
            i = indices[i].1;
        }
        (0..3)
            .map(|_| {
                (0..1000).for_each(|_| i = indices[i].1);
                self.numbers[i]
            })
            .sum()
    }
    fn part2(&self) -> Self::Answer2 {
        todo!()
    }
}

fn main() {
    let solution = Solution::new(std::io::stdin().lock());
    println!("Part 1: {}", solution.part1());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static [u8] {
        r"
1
2
-3
3
-2
0
4
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(3, Solution::new(example_input()).part1());
    }
}
