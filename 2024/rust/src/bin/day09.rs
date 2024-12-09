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
            sum += i / 2 * (2 * offsets[i] + disk_map[i] - 1) * disk_map[i] / 2;
            let mut len = disk_map[i];
            while let Some((j, space)) = disk_map
                .iter_mut()
                .enumerate()
                .skip(1)
                .step_by(2)
                .find(|(j, space)| *j < i && **space > (if move_whole { len - 1 } else { 0 }))
            {
                for _ in 0..len.min(*space) {
                    sum -= i / 2 * (offsets[i] - offsets[j] + len - 1);
                    *space -= 1;
                    offsets[j] += 1;
                    len -= 1;
                }
                if len == 0 {
                    break;
                }
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
        r"2333133121414131402
        "
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
