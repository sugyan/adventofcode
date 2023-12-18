use aoc2023::Solve;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{BufRead, BufReader, Read};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    N = 0,
    E = 1,
    S = 2,
    W = 3,
}

struct Solution {
    map: Vec<Vec<u32>>,
}

impl Solution {
    fn minimum_heat_loss(&self, ultra: bool) -> u32 {
        let (rows, cols) = (self.map.len(), self.map[0].len());
        let mut mins = vec![vec![[None; 4]; cols]; rows];
        let mut bh = BinaryHeap::from([
            (Reverse(0), (0, 0), Direction::E),
            (Reverse(0), (0, 0), Direction::S),
        ]);
        while let Some((Reverse(loss), (i, j), direction)) = bh.pop() {
            if i == rows - 1 && j == cols - 1 {
                return loss;
            }
            for d in match direction {
                Direction::N | Direction::S => [Direction::E, Direction::W],
                Direction::E | Direction::W => [Direction::N, Direction::S],
            } {
                let mut loss = loss;
                let (mut i, mut j) = (i, j);
                for blocks in 1..=10 {
                    (i, j) = match d {
                        Direction::N => (i.wrapping_sub(1), j),
                        Direction::E => (i, j.wrapping_add(1)),
                        Direction::S => (i.wrapping_add(1), j),
                        Direction::W => (i, j.wrapping_sub(1)),
                    };
                    if !(0..rows).contains(&i) || !(0..cols).contains(&j) {
                        break;
                    }
                    loss += self.map[i][j];
                    if (if ultra { 4..11 } else { 1..4 }).contains(&blocks)
                        && mins[i][j][d as usize].map_or(true, |min| loss < min)
                    {
                        mins[i][j][d as usize] = Some(loss);
                        bh.push((Reverse(loss), (i, j), d));
                    }
                }
            }
        }
        unreachable!();
    }
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        Self {
            map: BufReader::new(r)
                .lines()
                .map_while(Result::ok)
                .map(|s| {
                    s.chars()
                        .map(|c| c.to_digit(10).expect("should be a single digit"))
                        .collect()
                })
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.minimum_heat_loss(false)
    }
    fn part2(&self) -> Self::Answer2 {
        self.minimum_heat_loss(true)
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
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
"[1..]
            .as_bytes()
    }

    #[test]
    fn test_part1() {
        assert_eq!(Solution::new(example_input()).part1(), 102);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Solution::new(example_input()).part2(), 94);
        assert_eq!(
            Solution::new(
                r"
111111111111
999999999991
999999999991
999999999991
999999999991
"[1..]
                    .as_bytes()
            )
            .part2(),
            71
        );
    }
}
