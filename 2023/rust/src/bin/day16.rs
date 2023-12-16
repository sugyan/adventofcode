use aoc2023::Solve;
use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read};

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

struct Solution {
    contraption: Vec<Vec<char>>,
}

impl Solution {
    fn energized_count(&self, start: (usize, usize), direction: Direction) -> usize {
        let (rows, cols) = (self.contraption.len(), self.contraption[0].len());
        let mut seen = HashSet::new();
        let mut beams = vec![(start, direction)];
        while let Some(((i, j), direction)) = beams.pop() {
            let (i, j) = match direction {
                Direction::Right => (i, j.wrapping_add(1)),
                Direction::Up => (i.wrapping_add(!0), j),
                Direction::Left => (i, j.wrapping_add(!0)),
                Direction::Down => (i.wrapping_add(1), j),
            };
            if !(0..rows).contains(&i) || !(0..cols).contains(&j) {
                continue;
            }
            if !seen.insert(((i, j), direction)) {
                continue;
            }
            match (self.contraption[i][j], &direction) {
                ('|', Direction::Right | Direction::Left) => {
                    beams.push(((i, j), Direction::Up));
                    beams.push(((i, j), Direction::Down));
                }
                ('-', Direction::Up | Direction::Down) => {
                    beams.push(((i, j), Direction::Right));
                    beams.push(((i, j), Direction::Left));
                }
                ('/', d) => beams.push((
                    (i, j),
                    match d {
                        Direction::Right => Direction::Up,
                        Direction::Up => Direction::Right,
                        Direction::Left => Direction::Down,
                        Direction::Down => Direction::Left,
                    },
                )),
                ('\\', d) => {
                    beams.push((
                        (i, j),
                        match d {
                            Direction::Right => Direction::Down,
                            Direction::Up => Direction::Left,
                            Direction::Left => Direction::Up,
                            Direction::Down => Direction::Right,
                        },
                    ));
                }
                _ => beams.push(((i, j), direction)),
            }
        }
        seen.iter()
            .map(|((i, j), _)| (i, j))
            .collect::<HashSet<_>>()
            .len()
    }
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;

    fn new(r: impl Read) -> Self {
        Self {
            contraption: BufReader::new(r)
                .lines()
                .map_while(Result::ok)
                .map(|s| s.chars().collect())
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.energized_count((0, !0), Direction::Right)
    }
    fn part2(&self) -> Self::Answer2 {
        let (rows, cols) = (self.contraption.len(), self.contraption[0].len());
        [
            (0..cols)
                .map(|j| ((!0, j), Direction::Down))
                .collect::<Vec<_>>(),
            (0..cols)
                .map(|j| ((rows, j), Direction::Up))
                .collect::<Vec<_>>(),
            (0..rows)
                .map(|i| ((i, !0), Direction::Right))
                .collect::<Vec<_>>(),
            (0..rows)
                .map(|i| ((i, cols), Direction::Left))
                .collect::<Vec<_>>(),
        ]
        .concat()
        .iter()
        .map(|&(start, direction)| self.energized_count(start, direction))
        .max()
        .expect("should have max value")
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
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....        
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(Solution::new(example_input()).part1(), 46);
    }

    #[test]
    fn part2() {
        assert_eq!(Solution::new(example_input()).part2(), 51);
    }
}
