use itertools::Itertools;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

struct Solution {
    template: Vec<char>,
    rules: HashMap<(char, char), char>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
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
    fn part_1(&self) -> u64 {
        let counts = self.apply(10);
        counts.values().max().unwrap() - counts.values().min().unwrap()
    }
    fn part_2(&self) -> u64 {
        let counts = self.apply(40);
        counts.values().max().unwrap() - counts.values().min().unwrap()
    }
    fn apply(&self, steps: usize) -> HashMap<char, u64> {
        let mut pairs = HashMap::new();
        for w in self.template.windows(2) {
            *pairs.entry((w[0], w[1])).or_insert(0) += 1;
        }
        for _ in 0..steps {
            let mut hm = HashMap::new();
            for (&k, &v) in &pairs {
                let c = *self.rules.get(&k).unwrap();
                *hm.entry((k.0, c)).or_insert(0) += v;
                *hm.entry((c, k.1)).or_insert(0) += v;
            }
            pairs = hm;
        }
        let mut counts = HashMap::new();
        for (&k, &v) in &pairs {
            *counts.entry(k.0).or_insert(0) += v;
            *counts.entry(k.1).or_insert(0) += v;
        }
        counts.iter_mut().for_each(|(_, v)| *v = (*v + 1) / 2);
        counts
    }
}

fn main() {
    let solution = Solution::new(
        &BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(Result::ok)
            .collect::<Vec<_>>(),
    );
    println!("{}", solution.part_1());
    println!("{}", solution.part_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_inputs() -> Vec<String> {
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
CN -> C"[1..]
            .split('\n')
            .map(String::from)
            .collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(1588, Solution::new(&example_inputs()).part_1());
    }

    #[test]
    fn example_2() {
        assert_eq!(2_188_189_693_529, Solution::new(&example_inputs()).part_2());
    }
}
