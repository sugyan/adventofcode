use std::collections::HashMap;
use std::io::{BufRead, BufReader};

struct Solution {
    relationships: Vec<(String, String)>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        Self {
            relationships: inputs
                .iter()
                .map(|s| {
                    let v = s.split(')').collect::<Vec<_>>();
                    (v[0].to_string(), v[1].to_string())
                })
                .collect(),
        }
    }
    fn part_1(&self) -> usize {
        fn dfs(map: &HashMap<String, Vec<String>>, orbit: &str, depth: usize) -> usize {
            let mut ret = depth;
            if let Some(orbits) = map.get(orbit) {
                for o in orbits {
                    ret += dfs(map, o, depth + 1);
                }
            }
            ret
        }
        let mut map = HashMap::new();
        for relationship in &self.relationships {
            map.entry(relationship.0.clone())
                .or_insert_with(Vec::new)
                .push(relationship.1.clone());
        }
        dfs(&map, "COM", 0)
    }
    fn part_2(&self) -> usize {
        let mut map = HashMap::new();
        for relationship in &self.relationships {
            map.insert(relationship.1.clone(), relationship.0.clone());
        }
        let succ = |o: &String| map.get(o).cloned();
        let you = std::iter::successors(Some(String::from("YOU")), succ).collect::<Vec<_>>();
        let san = std::iter::successors(Some(String::from("SAN")), succ).collect::<Vec<_>>();
        if let Some(i) = (0..you.len().min(san.len()))
            .find(|&i| you[you.len() - 1 - i] != san[san.len() - 1 - i])
        {
            return you.len() + san.len() - i * 2 - 2;
        }
        unreachable!()
    }
}

fn main() {
    let solution = Solution::new(
        &BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(Result::ok)
            .collect::<Vec<_>>(),
    );
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            42,
            Solution::new(
                &r"
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L"
                .split('\n')
                .skip(1)
                .map(str::to_string)
                .collect::<Vec<_>>(),
            )
            .part_1()
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            4,
            Solution::new(
                &r"
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN"
                    .split('\n')
                    .skip(1)
                    .map(str::to_string)
                    .collect::<Vec<_>>(),
            )
            .part_2()
        );
    }
}
