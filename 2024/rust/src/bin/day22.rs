use aoc2024::{Solve, run};
use itertools::Itertools;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
}

struct Generator {
    secret_number: i64,
}

impl Generator {
    const MOD: i64 = 16777216;
}

impl Iterator for Generator {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        self.secret_number = ((self.secret_number << 6) ^ self.secret_number) % Self::MOD;
        self.secret_number = ((self.secret_number >> 5) ^ self.secret_number) % Self::MOD;
        self.secret_number = ((self.secret_number << 11) ^ self.secret_number) % Self::MOD;
        Some(self.secret_number)
    }
}

struct Solution {
    secret_numbers: Vec<i64>,
}

impl Solution {
    fn sales(secret_number: i64) -> HashMap<(i64, i64, i64, i64), i64> {
        let prices = [secret_number]
            .into_iter()
            .chain(Generator { secret_number }.take(2000))
            .map(|n| n % 10)
            .collect_vec();
        let mut ret = HashMap::new();
        for (i, sequence) in prices
            .windows(2)
            .map(|w| w[1] - w[0])
            .tuple_windows()
            .enumerate()
        {
            ret.entry(sequence).or_insert(prices[i + 4]);
        }
        ret
    }
}

impl Solve for Solution {
    type Answer1 = i64;
    type Answer2 = i64;
    type Error = Error;

    fn new<R>(r: R) -> Result<Self, Error>
    where
        R: Read,
    {
        Ok(Self {
            secret_numbers: BufReader::new(r)
                .lines()
                .map(|line| {
                    line.map_err(Error::Io)
                        .and_then(|s| s.parse().map_err(Error::Parse))
                })
                .collect::<Result<_, _>>()?,
        })
    }
    fn part1(&self) -> Self::Answer1 {
        self.secret_numbers
            .iter()
            .filter_map(|&secret_number| Generator { secret_number }.take(2000).last())
            .sum()
    }
    fn part2(&self) -> Self::Answer2 {
        let all_sales = self
            .secret_numbers
            .iter()
            .map(|&secret_number| Self::sales(secret_number))
            .collect_vec();
        all_sales
            .iter()
            .flat_map(|hm| hm.keys())
            .unique()
            .map(|sequence| {
                all_sales
                    .iter()
                    .filter_map(|hm| hm.get(sequence))
                    .sum::<i64>()
            })
            .fold(0, i64::max)
    }
}

fn main() -> Result<(), Error> {
    run::<Solution>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generator() {
        let mut generator = Generator { secret_number: 123 };
        assert_eq!(generator.next(), Some(15887950));
        assert_eq!(generator.next(), Some(16495136));
        assert_eq!(generator.next(), Some(527345));
        assert_eq!(generator.next(), Some(704524));
        assert_eq!(generator.next(), Some(1553684));
        assert_eq!(generator.next(), Some(12683156));
        assert_eq!(generator.next(), Some(11100544));
        assert_eq!(generator.next(), Some(12249484));
        assert_eq!(generator.next(), Some(7753432));
        assert_eq!(generator.next(), Some(5908254));
    }

    #[test]
    fn sales() {
        assert_eq!(Solution::sales(123).get(&(-1, -1, 0, 2)), Some(&6));
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(
            Solution::new(
                r"
1
10
100
2024
"[1..]
                    .as_bytes()
            )?
            .part1(),
            37327623
        );
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(
            Solution::new(
                r"
1
2
3
2024
"[1..]
                    .as_bytes()
            )?
            .part2(),
            23
        );
        Ok(())
    }
}
