use itertools::Itertools;
use std::collections::HashSet;
use std::io::{BufRead, BufReader};

enum Fold {
    X(u32),
    Y(u32),
}

struct Solution {
    dots: Vec<(u32, u32)>,
    folds: Vec<Fold>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
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
    fn part_1(&self) -> usize {
        self.fold(1).len()
    }
    fn part_2(&self) -> String {
        let dots = self.fold(self.folds.len());
        let xmax = dots.iter().map(|&(x, _)| x as usize).max().unwrap();
        let ymax = dots.iter().map(|&(_, y)| y as usize).max().unwrap();
        let mut paper = vec![vec!['.'; xmax + 1]; ymax + 1];
        dots.iter()
            .for_each(|&(x, y)| paper[y as usize][x as usize] = '#');
        paper
            .iter()
            .map(|row| row.iter().collect::<String>())
            .join("\n")
    }
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

fn main() {
    let solution = Solution::new(
        &BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(Result::ok)
            .collect::<Vec<_>>(),
    );
    println!("{}", solution.part_1());
    println!("{}", solution.part_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_inputs() -> Vec<String> {
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
fold along x=5"[1..]
            .split('\n')
            .map(String::from)
            .collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(17, Solution::new(&example_inputs()).part_1());
    }
}
