use aoc2022::Solve;
use std::io::{BufRead, BufReader, Read};

#[derive(Debug)]
enum Entry {
    File(u32),
    Directory(Vec<Entry>),
}

#[derive(Debug)]
struct FileSystem {
    root: Entry,
}

struct Solution {
    filesystem: FileSystem,
}

impl Solution {
    fn browse_around(curr: &mut Vec<Entry>, lines: &mut impl Iterator<Item = String>) {
        while let Some(line) = lines.next() {
            let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
            match *parts.first().unwrap() {
                "$" => {
                    if parts[1] == "cd" {
                        match *parts.get(2).unwrap() {
                            ".." => return,
                            "/" => {}
                            _ => {
                                let mut dir = Vec::new();
                                Self::browse_around(&mut dir, lines);
                                curr.push(Entry::Directory(dir));
                            }
                        }
                    }
                }
                s => {
                    if let Ok(size) = s.parse() {
                        curr.push(Entry::File(size));
                    }
                }
            }
        }
    }
    fn traverse(curr: &Entry, totals: &mut Vec<u32>) -> u32 {
        let mut total = 0;
        match curr {
            Entry::File(size) => return *size,
            Entry::Directory(dir) => {
                for entry in dir {
                    total += Self::traverse(entry, totals);
                }
            }
        }
        totals.push(total);
        total
    }
}

impl Solve for Solution {
    type Answer1 = u32;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        let mut root_dir = Vec::new();
        Self::browse_around(
            &mut root_dir,
            &mut BufReader::new(r).lines().filter_map(Result::ok),
        );
        Self {
            filesystem: FileSystem {
                root: Entry::Directory(root_dir),
            },
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let mut v = Vec::new();
        Self::traverse(&self.filesystem.root, &mut v);
        v.iter().filter(|&&x| x <= 100_000).sum()
    }
    fn part2(&self) -> Self::Answer2 {
        let mut v = Vec::new();
        Self::traverse(&self.filesystem.root, &mut v);
        *v.iter()
            .filter(|&&x| v[v.len() - 1] - x < 40_000_000)
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
