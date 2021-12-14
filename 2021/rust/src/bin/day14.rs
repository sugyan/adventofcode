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
    fn part_1(&self) -> u32 {
        let mut polymers = self.template.clone();
        for _ in 0..10 {
            for (j, i) in (1..polymers.len()).enumerate() {
                let c = *self
                    .rules
                    .get(&(polymers[i + j - 1], polymers[i + j]))
                    .unwrap();
                polymers.insert(i + j, c);
            }
        }
        let mut counts = polymers.iter().fold(vec![0; 26], |mut acc, &c| {
            acc[(c as u8 - b'A') as usize] += 1;
            acc
        });
        counts.sort_unstable();
        counts.reverse();
        while counts.last() == Some(&0) {
            counts.pop();
        }
        counts[0] - counts[counts.len() - 1]
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
}
