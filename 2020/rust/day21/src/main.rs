use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader};

struct Solution {
    data: Vec<(Vec<String>, Vec<String>)>,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        let re = Regex::new(r"^(.+?) \(contains (.*?)\)$").unwrap();
        let mut data = Vec::new();
        for line in inputs.iter() {
            if let Some(cap) = re.captures(line) {
                let ingredients = cap[1].split(' ').map(str::to_string).collect();
                let allergens = cap[2].split(", ").map(str::to_string).collect();
                data.push((ingredients, allergens));
            }
        }
        Self { data }
    }
    fn part_1(&self) -> usize {
        let candidates = self.calc_candidates();
        let mut candidate_ingredients = HashSet::new();
        for ingredients in candidates.values() {
            for ingredient in ingredients.iter() {
                candidate_ingredients.insert(ingredient);
            }
        }
        self.data
            .iter()
            .map(|(ingredients, _)| {
                ingredients
                    .iter()
                    .filter(|&ingredient| !candidate_ingredients.contains(ingredient))
                    .count()
            })
            .sum()
    }
    fn part_2(&self) -> String {
        let mut candidates = self.calc_candidates();
        let mut dangerous_ingredients = HashMap::with_capacity(candidates.len());
        while !candidates.is_empty() {
            let mut figure_outs = Vec::new();
            for (allergen, ingredients) in candidates
                .iter()
                .filter(|(_, ingredients)| ingredients.len() == 1)
            {
                dangerous_ingredients.insert(allergen.clone(), ingredients[0].clone());
                figure_outs.push(allergen.clone());
            }
            for allergen in figure_outs.iter() {
                if let Some(removed) = candidates.remove(allergen) {
                    candidates.values_mut().for_each(|ingredients| {
                        ingredients.retain(|ingredient| *ingredient != removed[0]);
                    });
                }
            }
        }
        let mut allergens = dangerous_ingredients.keys().collect::<Vec<_>>();
        allergens.sort_unstable();
        allergens
            .into_iter()
            .filter_map(|allergen| dangerous_ingredients.get(allergen))
            .map(String::to_string)
            .collect::<Vec<_>>()
            .join(",")
    }
    fn calc_candidates(&self) -> HashMap<String, Vec<String>> {
        let mut counts_map = HashMap::new();
        for (ingredients, allergens) in self.data.iter() {
            for allergen in allergens.iter() {
                for ingredient in ingredients.iter() {
                    *counts_map
                        .entry(allergen.to_string())
                        .or_insert_with(HashMap::new)
                        .entry(ingredient.to_string())
                        .or_insert(0) += 1;
                }
            }
        }
        let mut candidates = HashMap::new();
        for (allergen, counts) in counts_map.iter() {
            if let Some(&max) = counts.values().max() {
                candidates.insert(
                    allergen.to_string(),
                    counts
                        .iter()
                        .filter_map(|(ingredient, &count)| {
                            if count == max {
                                Some(ingredient.to_string())
                            } else {
                                None
                            }
                        })
                        .collect(),
                );
            }
        }
        candidates
    }
}

fn main() {
    let solution = Solution::new(
        BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(|line| line.ok())
            .collect(),
    );
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_inputs() -> Vec<String> {
        r"
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"
            .split('\n')
            .skip(1)
            .map(str::to_string)
            .collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(5, Solution::new(example_inputs()).part_1());
    }

    #[test]
    fn example_2() {
        assert_eq!(
            "mxmxvkd,sqjhc,fvjkl",
            Solution::new(example_inputs()).part_2()
        );
    }
}
