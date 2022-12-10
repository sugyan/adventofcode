use aoc2022::Solve;
use itertools::Itertools;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    values: [i32; 240],
}

impl Solution {}

impl Solve for Solution {
    type Answer1 = i32;
    type Answer2 = String;

    fn new(r: impl Read) -> Self {
        let mut iter = BufReader::new(r).lines().filter_map(Result::ok);
        let mut values = [0; 240];
        let (mut x, mut adding) = (1, None);
        for v in values.iter_mut() {
            *v = x;
            adding = if let Some(n) = adding {
                x += n;
                None
            } else {
                match iter.next().unwrap().as_str() {
                    "noop" => None,
                    s => Some(s[5..].parse::<i32>().unwrap()),
                }
            };
        }
        Self { values }
    }
    fn part1(&self) -> Self::Answer1 {
        [20, 60, 100, 140, 180, 220]
            .iter()
            .map(|&i| i as i32 * self.values[i - 1])
            .sum()
    }
    fn part2(&self) -> Self::Answer2 {
        String::from("\n")
            + &self
                .values
                .chunks(40)
                .map(|row| {
                    row.iter()
                        .enumerate()
                        .map(|(i, &x)| if (i as i32 - x).abs() < 2 { '#' } else { '.' })
                        .collect::<String>()
                })
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
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(13140, Solution::new(example_input()).part1());
    }

    #[test]
    fn part2() {
        assert_eq!(
            r"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....",
            Solution::new(example_input()).part2()
        );
    }
}
