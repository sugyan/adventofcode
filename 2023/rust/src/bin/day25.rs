use aoc2023::Solve;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{BufRead, BufReader, Read};

struct Solution {
    connections: Vec<(String, Vec<String>)>,
}

impl Solution {
    fn search_disconnection(
        graph: &HashMap<&String, Vec<String>>,
        (src, dst): (&String, &String),
        disconnect: &mut HashSet<(String, String)>,
    ) -> Option<usize> {
        let paths = Solution::search_paths(graph, (src, dst), disconnect);
        if let Some(path) = paths.get(dst) {
            if disconnect.len() >= 3 {
                return None;
            }
            for w in path.windows(2) {
                let d = (w[0].clone(), w[1].clone());
                disconnect.insert(d.clone());
                if let Some(len) = Self::search_disconnection(graph, (src, dst), disconnect) {
                    return Some(len);
                }
                disconnect.remove(&d);
            }
        } else {
            return Some(paths.len());
        }
        None
    }
    fn search_paths(
        graph: &HashMap<&String, Vec<String>>,
        (src, dst): (&String, &String),
        disconnect: &HashSet<(String, String)>,
    ) -> HashMap<String, Vec<String>> {
        let mut vd = VecDeque::from([src]);
        let mut paths = HashMap::from([(src.clone(), vec![src.clone()])]);
        while let Some(p) = vd.pop_front() {
            if p == dst {
                break;
            }
            for q in &graph[&p] {
                if paths.contains_key(q)
                    || disconnect.contains(&(p.clone(), q.clone()))
                    || disconnect.contains(&(q.clone(), p.clone()))
                {
                    continue;
                }
                paths.insert(
                    q.clone(),
                    paths[p].iter().chain(&[q.clone()]).cloned().collect(),
                );
                vd.push_back(q);
            }
        }
        paths
    }
}

impl Solve for Solution {
    type Answer1 = usize;
    type Answer2 = u32;

    fn new(r: impl Read) -> Self {
        Self {
            connections: BufReader::new(r)
                .lines()
                .map_while(Result::ok)
                .map(|line| {
                    let (name, others) = line.split_once(": ").expect("should be valid line");
                    (name.into(), others.split(' ').map(Into::into).collect())
                })
                .collect(),
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let mut graph = HashMap::new();
        for (name, others) in &self.connections {
            for other in others {
                graph
                    .entry(name)
                    .or_insert_with(Vec::new)
                    .push(other.clone());
                graph
                    .entry(other)
                    .or_insert_with(Vec::new)
                    .push(name.clone());
            }
        }
        for combination in graph.keys().combinations(2) {
            if let Some(len) = Self::search_disconnection(
                &graph,
                (combination[0], combination[1]),
                &mut HashSet::new(),
            ) {
                return len * (graph.len() - len);
            }
        }
        unreachable!();
    }
    fn part2(&self) -> Self::Answer2 {
        todo!()
    }
}

fn main() {
    let solution = Solution::new(std::io::stdin().lock());
    println!("Part 1: {}", solution.part1());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static [u8] {
        r"
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(Solution::new(example_input()).part1(), 54);
    }
}
