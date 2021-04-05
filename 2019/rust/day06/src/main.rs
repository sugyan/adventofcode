use std::collections::HashMap;
use std::io::{BufRead, BufReader};

struct Solution {
    map: HashMap<String, Vec<String>>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        let mut map = HashMap::new();
        for relationship in inputs.iter().map(|s| s.split(')').collect::<Vec<_>>()) {
            map.entry(relationship[0].to_string())
                .or_insert_with(Vec::new)
                .push(relationship[1].to_string())
        }
        Self { map }
    }
    fn part_1(&self) -> usize {
        fn dfs(map: &HashMap<String, Vec<String>>, orbit: String, depth: usize) -> usize {
            let mut ret = depth;
            if let Some(orbits) = map.get(&orbit) {
                for o in orbits {
                    ret += dfs(map, o.clone(), depth + 1);
                }
            }
            ret
        }
        dfs(&self.map, String::from("COM"), 0)
    }
    fn part_2(&self) -> i32 {
        unimplemented!()
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
}
