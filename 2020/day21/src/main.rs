use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader};

struct Solution {
    inputs: Vec<(Vec<String>, Vec<String>)>,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        let re = Regex::new(r"^(.+?) \(contains (.*?)\)$").unwrap();
        let mut data = Vec::new();
        for line in inputs.iter() {
            if let Some(cap) = re.captures(line) {
                let ingredients: Vec<String> = cap[1].split(' ').map(|s| s.to_string()).collect();
                let allergens: Vec<String> = cap[2].split(", ").map(|s| s.to_string()).collect();
                data.push((ingredients, allergens));
            }
        }
        Self { inputs: data }
    }
    fn solve_1(&self) -> usize {
        let candidates = self.candidates();
        let mut candidate_ingredients = HashSet::new();
        for ingredients in candidates.values() {
            for &ingredient in ingredients.iter() {
                candidate_ingredients.insert(ingredient);
            }
        }
        self.inputs
            .iter()
            .map(|(ingredients, _)| {
                ingredients
                    .iter()
                    .filter(|&ingredient| !candidate_ingredients.contains(ingredient.as_str()))
                    .count()
            })
            .sum()
    }
    fn solve_2(&self) -> String {
        let mut candidates = self.candidates();
        let mut dangerous_ingredients: HashMap<&str, &str> =
            HashMap::with_capacity(candidates.len());
        while !candidates.is_empty() {
            let mut figure_outs: Vec<&str> = Vec::new();
            for (&allergen, ingredients) in candidates
                .iter()
                .filter(|(_, ingredients)| ingredients.len() == 1)
            {
                dangerous_ingredients.insert(allergen, ingredients[0]);
                figure_outs.push(allergen);
            }
            for &allergen in figure_outs.iter() {
                if let Some(removed) = candidates.remove(allergen) {
                    candidates.values_mut().for_each(|ingredients| {
                        ingredients.retain(|&ingredient| ingredient != removed[0]);
                    });
                }
            }
        }
        let mut allergens: Vec<&&str> = dangerous_ingredients.keys().collect();
        allergens.sort_unstable();
        allergens
            .iter()
            .filter_map(|&allergen| dangerous_ingredients.get(allergen))
            .map(|&s| s.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
    fn candidates(&self) -> HashMap<&str, Vec<&str>> {
        let mut counts_map: HashMap<&str, HashMap<&str, usize>> = HashMap::new();
        for input in self.inputs.iter() {
            for allergen in input.1.iter() {
                for ingredient in input.0.iter() {
                    *counts_map
                        .entry(&allergen)
                        .or_insert_with(HashMap::new)
                        .entry(&ingredient)
                        .or_insert(0) += 1;
                }
            }
        }
        let mut candidates: HashMap<&str, Vec<&str>> = HashMap::new();
        for (allergen, counts) in counts_map.iter() {
            if let Some(&max) = counts.values().max() {
                candidates.insert(
                    allergen,
                    counts
                        .iter()
                        .filter(|(_, &count)| count == max)
                        .map(|(&ingredient, _)| ingredient)
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
