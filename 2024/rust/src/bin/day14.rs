use aoc2024::{Day, run_day};
use std::{cmp::Ordering, collections::HashSet, ops::Add, str::FromStr};
use thiserror::Error;

#[cfg(test)]
const SPACE: XY = XY { x: 11, y: 7 };
#[cfg(not(test))]
const SPACE: XY = XY { x: 101, y: 103 };

#[derive(Error, Debug)]
enum Error {
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

impl XY {
    fn wrap(self) -> Self {
        Self {
            x: self.x.rem_euclid(SPACE.x),
            y: self.y.rem_euclid(SPACE.y),
        }
    }
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
            x: self.x + rhs.x,
            y: self.y + rhs.y,
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

struct Input(Vec<Robot>);

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.lines().map(str::parse).collect::<Result<_, _>>()?))
    }
}

struct Solution;

impl Solution {
    #[allow(dead_code)]
    fn render(positions: &HashSet<XY>) {
        for y in 0..SPACE.y {
            for x in 0..SPACE.x {
                let c = if positions.contains(&XY { x, y }) {
                    '#'
                } else {
                    '.'
                };
                print!("{c}",);
            }
            println!();
        }
    }
}

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = usize;
    type Answer2 = usize;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        let mut quadrants = [[0; 2]; 2];
        for robot in &input.0 {
            let p = XY {
                x: robot.position.x + robot.velocity.x * 100,
                y: robot.position.y + robot.velocity.y * 100,
            }
            .wrap();
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
    fn part2(input: &Self::Input) -> Self::Answer2 {
        let mut robots = input.0.clone();
        for i in 0.. {
            let positions = robots.iter().map(|r| r.position).collect::<HashSet<_>>();
            if positions.len() == robots.len() {
                // Self::render(&positions); // for visual check
                return i;
            }
            for robot in &mut robots {
                robot.position = (robot.position + robot.velocity).wrap();
            }
        }
        unreachable!()
    }
}

fn main() -> Result<(), aoc2024::Error<Error>> {
    run_day::<Solution>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> Result<Input, Error> {
        r"
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
        .trim_start()
        .parse()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::part1(&example_input()?), 12);
        Ok(())
    }
}
