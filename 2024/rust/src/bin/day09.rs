use aoc2024::{Day, run_day};
use std::{array, cmp::Reverse, collections::BinaryHeap, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {}

struct Input(Vec<usize>);

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.trim().bytes().map(|u| usize::from(u - b'0')).collect(),
        ))
    }
}

#[derive(Debug)]
struct File {
    id: usize,
    position: usize,
    length: usize,
}

impl File {
    fn calculate_checksum(&self, pos: usize, len: usize) -> usize {
        self.id * (pos + pos + len - 1) * len / 2
    }
}

struct Solution;

impl Solution {
    fn extract_files_and_frees(input: &Input) -> (Vec<File>, [BinaryHeap<Reverse<usize>>; 10]) {
        let mut files = Vec::with_capacity(input.0.len() / 2);
        let mut frees = array::from_fn(|_| BinaryHeap::new());
        let mut pos = 0;
        for (i, digit) in input.0.iter().enumerate() {
            if i % 2 == 0 {
                files.push(File {
                    id: i / 2,
                    position: pos,
                    length: *digit,
                });
            } else {
                frees[*digit].push(Reverse(pos));
            }
            pos += *digit;
        }
        (files, frees)
    }
    fn resulting_checksum(input: &Input, move_whole: bool) -> usize {
        let (files, mut frees) = Self::extract_files_and_frees(input);
        let mut sum = 0;
        for file in files.iter().rev() {
            let mut remain = file.length;
            let need = if move_whole { file.length } else { 1 };
            while remain > 0 {
                if let Some((best_s, best_p)) = frees
                    .iter()
                    .enumerate()
                    .skip(need)
                    .filter_map(|(i, free)| {
                        free.peek()
                            .filter(|Reverse(p)| *p < file.position)
                            .map(|Reverse(p)| (i, *p))
                    })
                    .min_by_key(|(_, p)| *p)
                {
                    let take = best_s.min(remain);
                    sum += file.calculate_checksum(best_p, take);
                    remain -= take;
                    if let Some(Reverse(pos)) = frees[best_s].pop() {
                        frees[best_s - take].push(Reverse(pos + take));
                    }
                } else {
                    sum += file.calculate_checksum(file.position, remain);
                    break;
                }
            }
        }
        sum
    }
}

impl Day for Solution {
    type Input = Input;
    type Error = Error;
    type Answer1 = usize;
    type Answer2 = usize;

    fn part1(input: &Self::Input) -> Self::Answer1 {
        Self::resulting_checksum(input, false)
    }
    fn part2(input: &Self::Input) -> Self::Answer2 {
        Self::resulting_checksum(input, true)
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
2333133121414131402
"
        .trim_start()
        .parse()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::part1(&example_input()?), 1928);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::part2(&example_input()?), 2858);
        Ok(())
    }
}
