use aoc2023::Solve;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{BufRead, BufReader, Read};

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
enum Direction {
    N = 0,
    E = 1,
    S = 2,
    W = 3,
}

impl Direction {
    fn index(&self) -> usize {
        *self as usize
    }
    fn next_directions(&self) -> [Self; 3] {
        match self {
            Self::N => [Self::W, Self::N, Self::E],
            Self::E => [Self::N, Self::E, Self::S],
            Self::S => [Self::E, Self::S, Self::W],
            Self::W => [Self::S, Self::W, Self::N],
        }
    }
    fn next_position(&self, (i, j): (usize, usize)) -> (usize, usize) {
        match self {
            Self::N => (i.wrapping_sub(1), j),
            Self::E => (i, j.wrapping_add(1)),
            Self::S => (i.wrapping_add(1), j),
            Self::W => (i, j.wrapping_sub(1)),
        }
    }
}

struct Solution {
    map: Vec<Vec<u32>>,
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
        let (rows, cols) = (self.map.len(), self.map[0].len());
        let mut mins = vec![vec![[[None; 4]; 4]; cols]; rows];
        let mut bh = BinaryHeap::from([(Reverse(0), (0, 0), (Direction::E, 0))]);
        while let Some((Reverse(loss), (i, j), (direction, count))) = bh.pop() {
            if i == rows - 1 && j == cols - 1 {
                return loss;
            }
            for d in direction.next_directions() {
                if d == direction && count == 3 {
                    continue;
                }
                let (i, j) = d.next_position((i, j));
                if !(0..rows).contains(&i) || !(0..cols).contains(&j) {
                    continue;
                }
                let l = loss + self.map[i][j];
                let c = if d == direction { count + 1 } else { 1 };
                if mins[i][j][d.index()][c].map_or(true, |min| l < min) {
                    mins[i][j][d.index()][c] = Some(l);
                    bh.push((Reverse(l), (i, j), (d, c)));
                }
            }
        }
        unreachable!();
    }
    fn part2(&self) -> Self::Answer2 {
        todo!()
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
}
