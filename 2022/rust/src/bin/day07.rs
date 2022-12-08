use aoc2022::Solve;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    total_sizes: Vec<u32>,
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        let mut stack = Vec::new();
        let mut total_sizes = Vec::new();
        for line in BufReader::new(r).lines().filter_map(Result::ok) {
            match line.rsplit_once(char::is_whitespace) {
                Some(("$ cd", d)) => {
                    if d == ".." {
                        if let Some(total) = stack.pop() {
                            total_sizes.push(total);
                            if let Some(last) = stack.last_mut() {
                                *last += total;
                            }
                        }
                    } else {
                        stack.push(0);
                    }
                }
                Some((s, _)) => {
                    if let Ok(size) = s.parse::<u32>() {
                        if let Some(last) = stack.last_mut() {
                            *last += size;
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        while let Some(total) = stack.pop() {
            total_sizes.push(total);
            if let Some(last) = stack.last_mut() {
                *last += total;
            }
        }
        Self { total_sizes }
    }
    fn part1(&self) -> Self::Answer1 {
        self.total_sizes.iter().filter(|&x| *x <= 100_000).sum()
    }
    fn part2(&self) -> Self::Answer2 {
        *self
            .total_sizes
            .iter()
            .filter(|&x| self.total_sizes[self.total_sizes.len() - 1] - x < 40_000_000)
            .min()
            .unwrap()
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
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"[1..]
            .as_bytes()
    }

    #[test]
    fn example1() {
        assert_eq!(95437, Solution::new(example_input()).part1());
    }

    #[test]
    fn example2() {
        assert_eq!(24_933_642, Solution::new(example_input()).part2());
    }
}
