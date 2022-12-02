use aoc2021::Solve;
use itertools::Itertools;
use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read};

enum Fold {
    X(u32),
    Y(u32),
}

struct Solution {
    dots: Vec<(u32, u32)>,
    folds: Vec<Fold>,
}

impl Solution {
    fn fold(&self, end: usize) -> HashSet<(u32, u32)> {
        let mut dots = self.dots.iter().cloned().collect::<HashSet<_>>();
        for fold in &self.folds[..end] {
            dots = dots
                .iter()
                .map(|&(x, y)| match fold {
                    Fold::X(value) if x > *value => (value * 2 - x, y),
                    Fold::Y(value) if y > *value => (x, value * 2 - y),
                    _ => (x, y),
                })
                .collect::<HashSet<_>>()
        }
        dots
    }
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = String;

    fn new(r: impl Read) -> Self {
        let inputs = BufReader::new(r)
            .lines()
            .filter_map(Result::ok)
            .collect::<Vec<_>>();
        let mut parts = inputs.split(String::is_empty);
        let mut dots = Vec::new();
        if let Some(lines) = parts.next() {
            for line in lines {
                dots.push(
                    line.split(',')
                        .map(|s| s.parse().unwrap())
                        .collect_tuple()
                        .unwrap(),
                );
            }
        }
        let mut folds = Vec::new();
        if let Some(lines) = parts.next() {
            for line in lines {
                let (axis, value) = line[11..].split_once('=').unwrap();
                folds.push(match axis {
                    "x" => Fold::X(value.parse().unwrap()),
                    "y" => Fold::Y(value.parse().unwrap()),
                    _ => unreachable!(),
                });
            }
        }
        Self { dots, folds }
    }
    fn part1(&self) -> Self::Answer1 {
        self.fold(1).len()
    }
    fn part2(&self) -> Self::Answer2 {
        let dots = self.fold(self.folds.len());
        let xmax = dots.iter().map(|&(x, _)| x as usize).max().unwrap();
        let ymax = dots.iter().map(|&(_, y)| y as usize).max().unwrap();
        let mut paper = vec![vec!['.'; xmax + 1]; ymax + 1];
        dots.iter()
            .for_each(|&(x, y)| paper[y as usize][x as usize] = '#');
        String::from("\n")
            + &paper
                .iter()
                .map(|row| row.iter().collect::<String>())
                .join("\n")
    }
}

fn main() {
    let solution = Solution::new(std::io::stdin().lock());
    println!("Part 1: {}", solution.part1());
    println!("Part 2: {}", solution.part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static [u8] {
        r"
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
"[1..]
            .as_bytes()
    }

    #[test]
    fn example1() {
        assert_eq!(17, Solution::new(example_input()).part1());
    }
}
