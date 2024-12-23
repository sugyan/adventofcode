use aoc2024::{run, Solve};
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader, Read},
};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("invalid line")]
    InvalidLine,
}

struct Solution {
    graph: HashMap<String, HashSet<String>>,
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = usize;
    type Error = Error;

    fn new<R>(r: R) -> Result<Self, Error>
    where
        R: Read,
    {
        let mut graph = HashMap::new();
        for line in BufReader::new(r).lines() {
            let line = line?;
            let (name0, name1) = line.split_once('-').ok_or(Error::InvalidLine)?;
            graph
                .entry(name0.to_string())
                .or_insert_with(HashSet::new)
                .insert(name1.to_string());
            graph
                .entry(name1.to_string())
                .or_insert_with(HashSet::new)
                .insert(name0.to_string());
        }
        Ok(Self { graph })
    }
    fn part1(&self) -> Self::Answer1 {
        let mut hs = HashSet::new();
        for (k, v) in self.graph.iter().filter(|(k, _)| k.starts_with('t')) {
            for e0 in v {
                for e1 in self.graph[e0].intersection(v) {
                    hs.insert([k, e0, e1].into_iter().sorted().collect_vec());
                }
            }
        }
        hs.len()
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
        r"
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part1(), 7);
        Ok(())
    }
}
