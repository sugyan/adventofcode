use aoc2024::{Solve, run};
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

impl Solution {
    fn all_cliques(&self) -> Vec<Vec<String>> {
        let mut ret = Vec::new();
        self.bron_kerbosch(
            Vec::new(),
            &mut self.graph.keys().cloned().collect(),
            &mut HashSet::new(),
            &mut ret,
        );
        ret
    }
    fn bron_kerbosch(
        &self,
        r: Vec<String>,
        p: &mut HashSet<String>,
        x: &mut HashSet<String>,
        results: &mut Vec<Vec<String>>,
    ) {
        if p.is_empty() && x.is_empty() {
            return results.push(r);
        }
        for v in p.clone() {
            self.bron_kerbosch(
                r.iter().cloned().chain([v.clone()]).collect(),
                &mut p.intersection(&self.graph[&v]).cloned().collect(),
                &mut x.intersection(&self.graph[&v]).cloned().collect(),
                results,
            );
            p.remove(&v);
            x.insert(v);
        }
    }
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = String;
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
        self.all_cliques()
            .iter()
            .flat_map(|clique| {
                clique
                    .iter()
                    .combinations(3)
                    .filter(|c| c.iter().any(|s| s.starts_with('t')))
                    .map(|c| c.iter().cloned().sorted().collect_vec())
            })
            .unique()
            .count()
    }
    fn part2(&self) -> Self::Answer2 {
        self.all_cliques()
            .iter()
            .max_by_key(|v| v.len())
            .expect("no answer")
            .iter()
            .sorted()
            .join(",")
    }
}

fn main() -> Result<(), Error> {
    run::<Solution>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static [u8] {
        &r"
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
"
        .as_bytes()[1..]
    }

    #[test]
    fn part1() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part1(), 7);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), Error> {
        assert_eq!(Solution::new(example_input())?.part2(), "co,de,ka,ta");
        Ok(())
    }
}
