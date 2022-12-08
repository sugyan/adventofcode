use aoc2021::Solve;
use itertools::Itertools;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};

struct Solution {
    template: Vec<char>,
    rules: HashMap<(char, char), char>,
}

impl Solution {
    fn apply(&self, steps: usize) -> HashMap<char, u64> {
        let mut pairs = self.template.windows(2).fold(HashMap::new(), |mut hm, w| {
            *hm.entry((w[0], w[1])).or_insert(0) += 1;
            hm
        });
        for _ in 0..steps {
            pairs = pairs.iter().fold(HashMap::new(), |mut hm, (&k, &v)| {
                let c = self.rules[&k];
                *hm.entry((k.0, c)).or_insert(0) += v;
                *hm.entry((c, k.1)).or_insert(0) += v;
                hm
            });
        }
        let mut counts = pairs.iter().fold(HashMap::new(), |mut hm, (&k, &v)| {
            *hm.entry(k.0).or_insert(0) += v;
            *hm.entry(k.1).or_insert(0) += v;
            hm
        });
        counts.iter_mut().for_each(|(_, v)| *v = (*v + 1) / 2);
        counts
    }
}

impl Solve for Solution {
    type Answer1 = u64;
    type Answer2 = u64;

    fn new(r: impl Read) -> Self {
        let inputs = BufReader::new(r)
            .lines()
            .filter_map(Result::ok)
            .collect::<Vec<_>>();
        let mut rules = HashMap::new();
        for input in &inputs[2..] {
            let rule = input.split_once(" -> ").unwrap();
            rules.insert(
                rule.0.chars().collect_tuple().unwrap(),
                rule.1.chars().next().unwrap(),
            );
        }
        Self {
            template: inputs[0].chars().collect(),
            rules,
        }
    }
    fn part1(&self) -> Self::Answer1 {
        let counts = self.apply(10);
        counts.values().max().unwrap() - counts.values().min().unwrap()
    }
    fn part2(&self) -> Self::Answer2 {
        let counts = self.apply(40);
        counts.values().max().unwrap() - counts.values().min().unwrap()
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
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
"[1..]
            .as_bytes()
    }

    #[test]
    fn part1() {
        assert_eq!(1588, Solution::new(example_input()).part1());
    }

    #[test]
    fn part2() {
        assert_eq!(2_188_189_693_529, Solution::new(example_input()).part2());
    }
}
