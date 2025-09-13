use aoc2024::{Solve, run};
use std::{
    cmp::Ordering,
    collections::HashSet,
    io::{BufRead, BufReader, Read},
    ops::{Add, Mul},
    str::FromStr,
};
use thiserror::Error;

#[cfg(test)]
const SPACE: XY = XY { x: 11, y: 7 };
#[cfg(not(test))]
const SPACE: XY = XY { x: 101, y: 103 };

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
    #[error("invalid line")]
    InvalidLine,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct XY {
    x: i64,
    y: i64,
}

impl FromStr for XY {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(',')
            .ok_or(Error::InvalidLine)
            .and_then(|(x, y)| {
                Ok(Self {
                    x: x.parse()?,
                    y: y.parse()?,
                })
            })
    }
}

impl Add<Self> for XY {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: (self.x + rhs.x).rem_euclid(SPACE.x),
            y: (self.y + rhs.y).rem_euclid(SPACE.y),
        }
    }
}

impl Mul<i64> for XY {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[derive(Debug, Clone)]
struct Robot {
    position: XY,
    velocity: XY,
}

impl FromStr for Robot {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(' ')
            .ok_or(Error::InvalidLine)
            .and_then(|(p, v)| {
                Ok(Self {
                    position: p
                        .strip_prefix("p=")
                        .ok_or(Error::InvalidLine)
                        .and_then(|s| s.parse())?,
                    velocity: v
                        .strip_prefix("v=")
                        .ok_or(Error::InvalidLine)
                        .and_then(|s| s.parse())?,
                })
            })
    }
}

struct Solution {
    robots: Vec<Robot>,
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;
    type Error = Error;

    fn new<R>(r: R) -> Result<Self, Error>
    where
        R: Read,
    {
        Ok(Self {
            robots: BufReader::new(r)
                .lines()
                .map(|line| line?.parse())
                .collect::<Result<_, _>>()?,
        })
    }
    fn part1(&self) -> Self::Answer1 {
        let mut quadrants = [[0; 2]; 2];
        for robot in &self.robots {
            let p = robot.position + robot.velocity * 100;
            match (p.x.cmp(&(SPACE.x / 2)), p.y.cmp(&(SPACE.y / 2))) {
                (Ordering::Less, Ordering::Less) => quadrants[0][0] += 1,
                (Ordering::Greater, Ordering::Less) => quadrants[0][1] += 1,
                (Ordering::Less, Ordering::Greater) => quadrants[1][0] += 1,
                (Ordering::Greater, Ordering::Greater) => quadrants[1][1] += 1,
                _ => {}
            }
        }
        quadrants.iter().flatten().product()
    }
    fn part2(&self) -> Self::Answer2 {
        let mut robots = self.robots.clone();
        for i in 0.. {
            let positions = robots.iter().map(|r| r.position).collect::<HashSet<_>>();
            if positions.len() == robots.len() {
                // for y in 0..SPACE.y {
                //     for x in 0..SPACE.x {
                //         let c = if positions.contains(&XY { x, y }) {
                //             '#'
                //         } else {
                //             '.'
                //         };
                //         print!("{c}",);
                //     }
                //     println!();
                // }
                return i;
            }
            for robot in &mut robots {
                robot.position = robot.position + robot.velocity;
            }
        }
        unreachable!()
    }
}

fn main() -> Result<(), Error> {
    run::<Solution>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static [u8] {
        &r"
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
"
        .as_bytes()[1..]
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part1(), 12);
        Ok(())
    }
}
