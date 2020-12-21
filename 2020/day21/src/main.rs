use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader};

struct Solution {
    inputs: Vec<String>,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        Self { inputs }
    }
    fn solve_1(&self) -> usize {
        let re = Regex::new(r"^(.+?) \(contains (.*?)\)$").unwrap();
        let mut data: HashMap<String, Vec<Vec<String>>> = HashMap::new();
        let mut allergens: HashSet<String> = HashSet::new();
        for line in self.inputs.iter() {
            if let Some(cap) = re.captures(line) {
                let ingredients: Vec<String> = cap[1].split(' ').map(|s| s.to_string()).collect();
                data.entry(cap[2].to_string())
                    .or_insert_with(Vec::new)
                    .push(ingredients);
                for allergen in cap[2].split(", ") {
                    allergens.insert(allergen.to_string());
                }
            }
        }
        let mut hs: HashSet<String> = HashSet::new();
        for allergen in allergens.iter() {
            let mut threshold = 0;
            let mut counts: HashMap<&str, usize> = HashMap::new();
            for (k, v) in data.iter() {
                if k.split(", ").any(|s| s == allergen) {
                    threshold += 1;
                    for ingredients in v.iter() {
                        for ingredient in ingredients.iter() {
                            *counts.entry(&ingredient).or_insert(0) += 1;
                        }
                    }
                }
            }
            for (&s, &count) in counts.iter() {
                if count >= threshold {
                    hs.insert(s.to_string());
                }
            }
        }
        data.values()
            .map(|list| {
                list.iter()
                    .map(|ingredients| ingredients.iter().filter(|&s| !hs.contains(s)).count())
                    .sum::<usize>()
            })
            .sum()
    }
}

fn main() {
    let solution = Solution::new(
        BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(|line| line.ok())
            .collect(),
    );
    println!("{}", solution.solve_1());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            5,
            Solution::new(
                "
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"[1..]
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect()
            )
            .solve_1()
        );
    }
}
