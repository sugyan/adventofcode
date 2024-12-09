use aoc2024::{run, Solve};
use std::io::{BufReader, Read};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

struct Solution {
    disk_map: Vec<u8>,
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
            disk_map: buf.trim().bytes().map(|u| u - b'0').collect(),
        })
    }
    fn part1(&self) -> Self::Answer1 {
        let size = self.disk_map.iter().map(|u| *u as usize).sum::<usize>();
        let mut disk = vec![None; size];

        let mut offset = 0;
        for (i, len) in self.disk_map.iter().enumerate() {
            if i % 2 == 0 {
                for j in 0..*len as usize {
                    disk[offset + j] = Some(i / 2);
                }
            }
            offset += *len as usize;
        }
        while let Some(i) = disk.iter().position(|u| u.is_none()) {
            while let Some(last) = disk.pop() {
                if last.is_none() {
                    continue;
                }
                if i < disk.len() {
                    disk[i] = last;
                } else {
                    disk.push(last)
                }
                break;
            }
        }
        disk.into_iter()
            .flatten()
            .enumerate()
            .map(|(i, u)| i * u)
            .sum()
    }
    fn part2(&self) -> Self::Answer2 {
        todo!()
    }
}

fn main() -> Result<(), Error> {
    run::<Solution>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static [u8] {
        r"2333133121414131402
        "
        .as_bytes()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part1(), 1928);
        Ok(())
    }
}
