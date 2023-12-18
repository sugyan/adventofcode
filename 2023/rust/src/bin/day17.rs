use aoc2023::Solve;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::io::{BufRead, BufReader, Read};

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn next_directions(&self) -> [Self; 3] {
        match self {
            Self::N => [Self::W, Self::N, Self::E],
            Self::E => [Self::N, Self::E, Self::S],
            Self::S => [Self::E, Self::S, Self::W],
            Self::W => [Self::S, Self::W, Self::N],
        }
    }
    fn next_positions(&self, (mut i, mut j): (usize, usize), steps: usize) -> Vec<(usize, usize)> {
        let mut ret = Vec::new();
        for _ in 0..steps {
            (i, j) = match self {
                Self::N => (i.wrapping_sub(1), j),
                Self::E => (i, j.wrapping_add(1)),
                Self::S => (i.wrapping_add(1), j),
                Self::W => (i, j.wrapping_sub(1)),
            };
            ret.push((i, j));
        }
        ret
    }
}

struct Solution {
    map: Vec<Vec<u32>>,
}

impl Solution {
    fn minimum_heat_loss(&self, ultra: bool) -> u32 {
        let (rows, cols) = (self.map.len(), self.map[0].len());
        let mut mins = vec![vec![HashMap::new(); cols]; rows];
        let mut bh = BinaryHeap::from([
            (Reverse(0), (0, 0), (Direction::N, 0)),
            (Reverse(0), (0, 0), (Direction::W, 0)),
        ]);
        while let Some((Reverse(loss), (i, j), (direction, count))) = bh.pop() {
            if i == rows - 1 && j == cols - 1 {
                return loss;
            }
            for d in direction.next_directions() {
                if d == direction && count == if ultra { 10 } else { 3 } {
                    continue;
                }
                let steps = if ultra && d != direction { 4 } else { 1 };
                let positions = d.next_positions((i, j), steps);
                let &(i, j) = positions.last().expect("should have at least one position");
                if !(0..rows).contains(&i) || !(0..cols).contains(&j) {
                    continue;
                }
                let l = loss + positions.iter().map(|&(i, j)| self.map[i][j]).sum::<u32>();
                let c = if d == direction { count + 1 } else { steps };
                if mins[i][j].get(&(d, c)).map_or(true, |&min| l < min) {
                    mins[i][j].insert((d, c), l);
                    bh.push((Reverse(l), (i, j), (d, c)));
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
