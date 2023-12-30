use aoc2023::Solve;
use itertools::{iproduct, Itertools};
use std::io::{BufRead, BufReader, Read};
use std::ops::{Add, Div, Mul, RangeInclusive, Sub};

type Position2 = (i128, i128);
type Position3 = (i128, i128, i128);
type Velocity2 = (i128, i128);
type Velocity3 = (i128, i128, i128);

fn gcd(a: i128, b: i128) -> i128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[derive(Clone, Copy)]
struct Rational {
    numer: i128,
    denom: i128,
}

impl Rational {
    fn new(numer: i128, denom: i128) -> Self {
        let sign = if numer.signum() * denom.signum() < 0 {
            -1
        } else {
            1
        };
        if numer == 0 && denom == 0 {
            Self { numer: 0, denom: 1 }
        } else {
            let g = gcd(numer.abs(), denom.abs());
            Self {
                numer: numer.abs() / g * sign,
                denom: denom.abs() / g,
            }
        }
    }
    fn signum(&self) -> i128 {
        self.numer.signum() * self.denom.signum()
    }
}

impl From<i128> for Rational {
    fn from(n: i128) -> Self {
        Self::new(n, 1)
    }
}

impl Add for Rational {
    type Output = Rational;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.numer * rhs.denom + rhs.numer * self.denom,
            self.denom * rhs.denom,
        )
    }
}

impl Sub for Rational {
    type Output = Rational;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.numer * rhs.denom - rhs.numer * self.denom,
            self.denom * rhs.denom,
        )
    }
}

impl Mul for Rational {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.numer * rhs.numer, self.denom * rhs.denom)
    }
}

impl Div for Rational {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.numer * rhs.denom, self.denom * rhs.numer)
    }
}

impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        self.numer * other.denom == other.numer * self.denom
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.numer * other.denom).partial_cmp(&(other.numer * self.denom))
    }
}

enum Line {
    Slope(Rational, Rational),
    X(Rational),
}

struct Solution {
    hailstones: Vec<(Position3, Velocity3)>,
}

impl Solution {
    fn count_intersections(&self, test_area: (i128, i128)) -> usize {
        let min = Rational::from(test_area.0);
        let max = Rational::from(test_area.1);
        self.hailstones
            .iter()
            .combinations(2)
            .filter(|c| {
                let &((px0, py0, _), (vx0, vy0, _)) = c[0];
                let &((px1, py1, _), (vx1, vy1, _)) = c[1];
                let a0 = Rational::from(vy0) / Rational::from(vx0);
                let a1 = Rational::from(vy1) / Rational::from(vx1);
                let b0 = Rational::from(py0) - a0 * Rational::from(px0);
                let b1 = Rational::from(py1) - a1 * Rational::from(px1);
                let (x, y) = Self::intersection_point((b0, a0), (b1, a1));
                (min <= x && x <= max && min <= y && y <= max)
                    && ((x - px0.into()).signum() == Rational::from(vx0).signum())
                    && ((x - px1.into()).signum() == Rational::from(vx1).signum())
            })
            .count()
    }
    fn single_throw_position(&self, test_area: (i128, i128)) -> i64 {
        let xy = Self::search_velocities(
            &self
                .hailstones
                .iter()
                .map(|&((hpx, hpy, _), (hvx, hvy, _))| ((hpx, hpy), (hvx, hvy)))
                .collect_vec(),
            test_area.0..=test_area.1,
        );
        let yz = Self::search_velocities(
            &self
                .hailstones
                .iter()
                .map(|&((_, hpy, hpz), (_, hvy, hvz))| ((hpy, hpz), (hvy, hvz)))
                .collect_vec(),
            test_area.0..=test_area.1,
        );
        let xz = Self::search_velocities(
            &self
                .hailstones
                .iter()
                .map(|&((hpx, _, hpz), (hvx, _, hvz))| ((hpx, hpz), (hvx, hvz)))
                .collect_vec(),
            test_area.0..=test_area.1,
        );
        assert!(xy.len() == 1 && yz.len() == 1 && xz.len() == 1);
        let ((vx0, vy0), (x0, y0)) = xy[0];
        let ((vy1, vz1), (y1, z1)) = yz[0];
        let ((vx2, vz2), (x2, z2)) = xz[0];
        assert!(vx0 == vx2 && vy0 == vy1 && vz1 == vz2);
        assert!(x0 == x2 && y0 == y1 && z1 == z2);
        [
            (x0.numer / x0.denom) as i64,
            (y0.numer / y0.denom) as i64,
            (z1.numer / z1.denom) as i64,
        ]
        .iter()
        .sum()
    }
    fn intersection_point(
        (c0, v0): (Rational, Rational),
        (c1, v1): (Rational, Rational),
    ) -> (Rational, Rational) {
        // c0 + v0 * t = c1 + v1 * t
        let t = (c1 - c0) / (v0 - v1);
        (t, c0 + v0 * t)
    }
    fn search_velocities(
        hailstones: &[(Position2, Velocity2)],
        range: RangeInclusive<i128>,
    ) -> Vec<(Velocity2, (Rational, Rational))> {
        let mut ret = Vec::new();
        for (vx, vy) in iproduct!(range.clone(), range.clone()) {
            let lines = hailstones
                .iter()
                .map(|&((hpx, hpy), (hvx, hvy))| {
                    if hvx == vx {
                        Line::X(Rational::from(hpx))
                    } else {
                        let a = Rational::new(hvy - vy, hvx - vx);
                        let b = Rational::from(hpy) - a * Rational::from(hpx);
                        Line::Slope(a, b)
                    }
                })
                .collect_vec();
            if let Some((x0, y0)) = match (&lines[0], &lines[1]) {
                (Line::Slope(a0, b0), Line::Slope(a1, b1)) if a0 != a1 => {
                    Some(Self::intersection_point((*b0, *a0), (*b1, *a1)))
                }
                (Line::Slope(a, b), Line::X(x)) | (Line::X(x), Line::Slope(a, b)) => {
                    Some((*x, *a * *x + *b))
                }
                _ => None,
            } {
                if lines.iter().all(|line| match line {
                    Line::Slope(a, b) => *a * x0 + *b == y0,
                    Line::X(x) => *x == x0,
                }) {
                    ret.push(((vx, vy), (x0, y0)));
                }
            }
        }
        ret
    }
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = i64;

    fn new(r: impl Read) -> Self {
        Self {
            hailstones: BufReader::new(r)
                .lines()
                .map_while(Result::ok)
                .map(|line| {
                    let (pos, vel) = line.split_once(" @ ").expect("should be valid line");
                    let parse = |s: &str| {
                        s.split(", ")
                            .map(|s| s.trim().parse().expect("should be valid number"))
                            .collect_tuple()
                            .expect("should be valid tuple")
                    };
                    (parse(pos), parse(vel))
                })
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.count_intersections((200_000_000_000_000, 400_000_000_000_000))
    }
    fn part2(&self) -> Self::Answer2 {
        self.single_throw_position((-500, 500))
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
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(
            Solution::new(example_input()).count_intersections((7, 27)),
            2
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            Solution::new(example_input()).single_throw_position((-10, 10)),
            47
        );
    }
}
