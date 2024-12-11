use aoc2024::{run, Solve};
use std::io::{BufReader, Read};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

struct Solution {
    disk_map: Vec<usize>,
}

impl Solution {
    fn resulting_checksum(&self, move_whole: bool) -> usize {
        let mut disk_map = self.disk_map.clone();
        let mut offsets = vec![0; disk_map.len()];
        for i in 1..disk_map.len() {
            offsets[i] = offsets[i - 1] + disk_map[i - 1];
        }
        let mut sum = 0;
        for i in (0..self.disk_map.len()).step_by(2).rev() {
            while let Some(j) = (1..i)
                .step_by(2)
                .find(|j| disk_map[*j] > if move_whole { disk_map[i] - 1 } else { 0 })
            {
                for _ in 0..disk_map[i].min(disk_map[j]) {
                    sum += i / 2 * offsets[j];
                    offsets[j] += 1;
                    disk_map[i] -= 1;
                    disk_map[j] -= 1;
                }
                if disk_map[i] == 0 {
                    break;
                }
            }
            for j in 0..disk_map[i] {
                sum += i / 2 * (offsets[i] + j);
            }
        }
        sum
    }
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;
    type Error = Error;

    fn new<R>(r: R) -> Result<Self, Error>
    where
        R: Read,
    {
        let mut buf = String::new();
        BufReader::new(r).read_to_string(&mut buf)?;
        Ok(Self {
            disk_map: buf.trim().bytes().map(|u| usize::from(u - b'0')).collect(),
        })
    }
    fn part1(&self) -> Self::Answer1 {
        self.resulting_checksum(false)
    }
    fn part2(&self) -> Self::Answer2 {
        self.resulting_checksum(true)
    }
}

fn main() -> Result<(), Error> {
    run::<Solution>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static [u8] {
        r"
2333133121414131402
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part1(), 1928);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part2(), 2858);
        Ok(())
    }
}
