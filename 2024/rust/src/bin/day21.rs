use aoc2024::{run, Solve};
use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::HashMap,
    io::{BufRead, BufReader, Read},
};
use thiserror::Error;

const DIRECTIONALS: [((char, char), &str); 25] = [
    (('A', 'A'), ""),
    (('A', '^'), "<"),
    (('A', '>'), "v"),
    (('A', 'v'), "<v"),
    (('A', '<'), "v<<"),
    (('^', 'A'), ">"),
    (('^', '^'), ""),
    (('^', '>'), "v>"),
    (('^', 'v'), "v"),
    (('^', '<'), "v<"),
    (('>', '^'), "<^"),
    (('>', 'A'), "^"),
    (('>', '>'), ""),
    (('>', 'v'), "<"),
    (('>', '<'), "<<"),
    (('v', '^'), "^"),
    (('v', 'A'), "^>"),
    (('v', '>'), ">"),
    (('v', 'v'), ""),
    (('v', '<'), "<"),
    (('<', '^'), ">^"),
    (('<', 'A'), ">>^"),
    (('<', '>'), ">>"),
    (('<', 'v'), ">"),
    (('<', '<'), ""),
];

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

struct Solution {
    codes: Vec<String>,
}

impl Solution {
    fn sum_of_complexities(&self, robots: usize) -> usize {
        let paths = Self::build_paths();
        let directional_map = DIRECTIONALS.into_iter().collect::<HashMap<_, _>>();
        let mut sum = 0;
        for code in &self.codes {
            if let Some(min) = Self::candidates(code, &paths)
                .iter()
                .map(|keys| {
                    let mut counts = format!("A{keys}")
                        .chars()
                        .tuple_windows::<(_, _)>()
                        .counts();
                    for _ in 0..robots {
                        counts = counts
                            .iter()
                            .flat_map(|(key, count)| {
                                format!("A{}A", directional_map[key])
                                    .chars()
                                    .tuple_windows::<(_, _)>()
                                    .counts()
                                    .into_iter()
                                    .map(|(k, v)| (k, v * *count))
                            })
                            .into_grouping_map()
                            .sum();
                    }
                    counts.values().sum::<usize>()
                })
                .min()
            {
                sum += min
                    * code
                        .strip_suffix('A')
                        .and_then(|s| s.parse::<usize>().ok())
                        .expect("invalid code");
            }
        }
        sum
    }
    fn build_paths() -> HashMap<(char, char), Vec<String>> {
        let positions = [
            vec![Some('7'), Some('8'), Some('9')],
            vec![Some('4'), Some('5'), Some('6')],
            vec![Some('1'), Some('2'), Some('3')],
            vec![None, Some('0'), Some('A')],
        ]
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &c)| ((i, j), c)))
        .collect::<HashMap<_, _>>();
        let mut paths = HashMap::new();
        for ((i0, j0), src) in positions.iter() {
            for ((i1, j1), dst) in positions.iter() {
                let (Some(src), Some(dst)) = (src, dst) else {
                    continue;
                };
                if src == dst {
                    continue;
                }
                let v = if i0 == i1 || j0 == j1 {
                    vec![vec![(i0, j0), (i1, j1)]]
                } else {
                    let mut ret = Vec::new();
                    if positions[&(*i0, *j1)].is_some() {
                        ret.push(vec![(i0, j0), (i0, j1), (i1, j1)]);
                    }
                    if positions[&(*i1, *j0)].is_some() {
                        ret.push(vec![(i0, j0), (i1, j0), (i1, j1)]);
                    }
                    ret
                };
                paths.insert(
                    (*src, *dst),
                    v.iter()
                        .map(|ps| {
                            ps.windows(2)
                                .map(|w| {
                                    use Ordering::*;
                                    match (w[0].0.cmp(w[1].0), w[0].1.cmp(w[1].1)) {
                                        (Less, _) => vec!["v"; w[1].0 - w[0].0].join(""),
                                        (Greater, _) => vec!["^"; w[0].0 - w[1].0].join(""),
                                        (_, Less) => vec![">"; w[1].1 - w[0].1].join(""),
                                        (_, Greater) => vec!["<"; w[0].1 - w[1].1].join(""),
                                        _ => unreachable!(),
                                    }
                                })
                                .join("")
                        })
                        .collect_vec(),
                );
            }
        }
        paths
    }
    fn candidates(s: &str, paths: &HashMap<(char, char), Vec<String>>) -> Vec<String> {
        let mut v = vec![String::new()];
        for cs in format!("A{s}").chars().tuple_windows() {
            v = v
                .into_iter()
                .flat_map(|s| {
                    paths[&cs]
                        .iter()
                        .map(|seq| format!("{s}{seq}"))
                        .collect::<Vec<_>>()
                })
                .map(|s| format!("{s}A"))
                .collect();
        }
        v
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
        Ok(Self {
            codes: BufReader::new(r).lines().collect::<Result<_, _>>()?,
        })
    }
    fn part1(&self) -> Self::Answer1 {
        self.sum_of_complexities(2)
    }
    fn part2(&self) -> Self::Answer2 {
        self.sum_of_complexities(25)
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
029A
980A
179A
456A
379A
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part1(), 126384);
        Ok(())
    }
}
