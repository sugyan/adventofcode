use aoc2023::Solve;
use itertools::Itertools;
use std::io::{BufRead, BufReader, Read};
use std::ops::Range;

struct Solution {
    seeds: Vec<i64>,
    maps: Vec<Vec<(Range<i64>, i64)>>,
}

impl Solution {
    fn lowest_location(&self, mut ranges: Vec<Range<i64>>) -> i64 {
        for map in &self.maps {
            let search = |n: i64| {
                map.binary_search_by(|(r, _)| {
                    if n < r.start {
                        std::cmp::Ordering::Greater
                    } else if n >= r.end {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Equal
                    }
                })
            };
            ranges = ranges
                .iter()
                .flat_map(|range| {
                    let start = search(range.start);
                    let end = search(range.end);
                    if let (Err(i), Err(j)) = (start, end) {
                        if i == j || i == map.len() || j == 0 {
                            return vec![range.clone()];
                        }
                    }
                    let mut v = Vec::new();
                    let i = match start {
                        Ok(i) => i,
                        Err(i) => {
                            v.push(range.start..map[i].0.start);
                            i
                        }
                    };
                    let j = match end {
                        Ok(j) => j,
                        Err(j) => {
                            v.push(map[j - 1].0.end..range.end);
                            j - 1
                        }
                    };
                    for k in i..=j {
                        if k < j && map[k].0.end < map[k + 1].0.start {
                            v.push(map[k].0.end..map[k + 1].0.start);
                        }
                        v.push(
                            map[k].0.start.max(range.start) + map[k].1
                                ..map[k].0.end.min(range.end) + map[k].1,
                        );
                    }
                    v
                })
                .collect();
        }
        ranges.iter().map(|r| r.start).min().unwrap()
    }
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
                        .map(|(dst, src, len)| ((src..src + len), dst - src))
                        .collect()
                })
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        self.lowest_location(self.seeds.iter().map(|&seed| seed..seed + 1).collect())
    }
    fn part2(&self) -> Self::Answer2 {
        self.lowest_location(
            self.seeds
                .chunks(2)
                .map(|chunk| chunk[0]..chunk[0] + chunk[1])
                .collect(),
        )
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

    #[test]
    fn part2() {
        assert_eq!(Solution::new(example_input()).part2(), 46);
    }
}
