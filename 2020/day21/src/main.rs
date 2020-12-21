use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader};

struct Solution {
    allergens: HashSet<String>,
    data: HashMap<String, Vec<Vec<String>>>,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        let re = Regex::new(r"^(.+?) \(contains (.*?)\)$").unwrap();
        let mut data: HashMap<String, Vec<Vec<String>>> = HashMap::new();
        let mut allergens: HashSet<String> = HashSet::new();
        for line in inputs.iter() {
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
        Self { allergens, data }
    }
    fn solve_1(&self) -> usize {
        let mut hs: HashSet<String> = HashSet::new();
        for allergen in self.allergens.iter() {
            let mut threshold = 0;
            let mut counts: HashMap<&str, usize> = HashMap::new();
            for (k, v) in self.data.iter() {
                if k.split(", ").any(|s| s == allergen) {
                    for ingredients in v.iter() {
                        threshold += 1;
                        for ingredient in ingredients.iter() {
                            *counts.entry(&ingredient).or_insert(0) += 1;
                        }
                    }
                }
            }
            for (&s, &count) in counts.iter() {
                if count == threshold {
                    hs.insert(s.to_string());
                }
            }
        }
        self.data
            .values()
            .map(|list| {
                list.iter()
                    .map(|ingredients| ingredients.iter().filter(|&s| !hs.contains(s)).count())
                    .sum::<usize>()
            })
            .sum()
    }
    fn solve_2(&self) -> String {
        let mut candidates: HashMap<&str, HashSet<&str>> = HashMap::new();
        for allergen in self.allergens.iter() {
            let mut threshold = 0;
            let mut counts: HashMap<&str, usize> = HashMap::new();
            for (k, v) in self.data.iter() {
                if k.split(", ").any(|s| s == allergen) {
                    for ingredients in v.iter() {
                        threshold += 1;
                        for ingredient in ingredients.iter() {
                            *counts.entry(&ingredient).or_insert(0) += 1;
                        }
                    }
                }
            }
            candidates.insert(
                &allergen,
                counts
                    .iter()
                    .filter(|(_, &count)| count == threshold)
                    .map(|(&s, _)| s)
                    .collect(),
            );
        }
        let mut ingredients: HashMap<&str, &str> = HashMap::with_capacity(candidates.len());
        while !candidates.is_empty() {
            let mut v: Vec<&str> = Vec::new();
            for (&allergen, set) in candidates.iter().filter(|(_, set)| set.len() == 1) {
                if let Some(ingredient) = set.iter().next() {
                    ingredients.insert(allergen, ingredient);
                    v.push(allergen);
                }
            }
            for &allergen in v.iter() {
                if let Some(set) = candidates.remove(allergen) {
                    if let Some(&removed) = set.iter().next() {
                        candidates.values_mut().for_each(|set| {
                            set.retain(|&ingredient| ingredient != removed);
                        });
                    }
                }
            }
        }
        let mut allergens: Vec<&str> = self.allergens.iter().map(|s| s.as_str()).collect();
        allergens.sort_unstable();
        allergens
            .iter()
            .filter_map(|&allergen| ingredients.get(allergen))
            .map(|&s| s.to_string())
            .collect::<Vec<String>>()
            .join(",")
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
    println!("{}", solution.solve_2());
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

    #[test]
    fn example_2() {
        assert_eq!(
            "mxmxvkd,sqjhc,fvjkl",
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
            .solve_2()
        );
    }
}
