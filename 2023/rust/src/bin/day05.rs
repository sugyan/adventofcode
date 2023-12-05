use aoc2023::Solve;
use itertools::Itertools;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    seeds: Vec<i64>,
    maps: Vec<Vec<(i64, i64, i64)>>,
}

impl Solve for Solution {
    type Answer1 = i64;
    type Answer2 = i64;

    fn new(r: impl Read) -> Self {
        let lines = BufReader::new(r)
            .lines()
            .map_while(Result::ok)
            .collect_vec();
        let parts = lines.split(String::is_empty).collect_vec();
        Self {
            seeds: parts[0][0]
                .strip_prefix("seeds: ")
                .unwrap()
                .split(' ')
                .map(|s| s.parse().unwrap())
                .collect(),
            maps: parts[1..]
                .iter()
                .map(|lines| {
                    lines[1..]
                        .iter()
                        .map(|line| {
                            line.split(' ')
                                .map(|s| s.parse().unwrap())
                                .collect_tuple()
                                .unwrap()
                        })
                        .sorted_by_cached_key(|(_, src, _)| *src)
                        .collect()
                })
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let mut ret = i64::MAX;
        for seed in &self.seeds {
            let mut n = *seed;
            for m in &self.maps {
                let ret = m.binary_search_by_key(&n, |(_, src, _)| *src);
                let (dst, src, len) = match ret {
                    Ok(i) => m[i],
                    Err(i) => m[i.max(1) - 1],
                };
                if n >= src && n < src + len {
                    n += dst - src;
                }
            }
            ret = ret.min(n);
        }
        ret
    }
    fn part2(&self) -> Self::Answer2 {
        todo!()
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
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(Solution::new(example_input()).part1(), 35);
    }
}
