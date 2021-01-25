use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader};

struct Solution {
    rules: Vec<String>,
}

impl Solution {
    fn new(inputs: &[String]) -> Self {
        Self {
            rules: inputs.iter().map(String::to_string).collect(),
        }
    }
    fn part_1(&self) -> usize {
        let mut hm = HashMap::new();
        for line in &self.rules {
            let v = line.split(" bags contain ").collect::<Vec<_>>();
            if v[1] != "no other bags." {
                for s in v[1].split(", ") {
                    let l = s.find(' ').unwrap();
                    let r = s.rfind(' ').unwrap();
                    hm.entry(&s[l + 1..r]).or_insert_with(Vec::new).push(v[0]);
                }
            }
        }
        let mut hs = HashSet::new();
        let mut stack = vec!["shiny gold"];
        while let Some(last) = stack.pop() {
            if !hs.contains(last) {
                hs.insert(last);
                if let Some(set) = hm.get(last) {
                    for &s in set {
                        stack.push(s);
                    }
                }
            }
        }
        hs.len() - 1
    }
    fn part_2(&self) -> usize {
        let mut hm = HashMap::new();
        for line in &self.rules {
            let v = line.split(" bags contain ").collect::<Vec<_>>();
            hm.insert(
                v[0],
                if v[1] == "no other bags." {
                    Vec::new()
                } else {
                    v[1].split(", ")
                        .map(|s| {
                            let l = s.find(' ').unwrap();
                            let r = s.rfind(' ').unwrap();
                            let n: usize = s[..l].parse().unwrap();
                            (n, &s[l + 1..r])
                        })
                        .collect()
                },
            );
        }
        let mut ret = 0;
        let mut stack = vec![(1, "shiny gold")];
        while let Some(last) = stack.pop() {
            ret += last.0;
            if let Some(v) = hm.get(last.1) {
                for &e in v.iter() {
                    stack.push((last.0 * e.0, e.1));
                }
            }
        }
        ret - 1
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

    fn example_inputs_1() -> Vec<String> {
        r"
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."
            .split('\n')
            .skip(1)
            .map(str::to_string)
            .collect()
    }

    fn example_inputs_2() -> Vec<String> {
        r"
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."
            .split('\n')
            .skip(1)
            .map(str::to_string)
            .collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(4, Solution::new(&example_inputs_1()).part_1());
    }

    #[test]
    fn example_2() {
        assert_eq!(32, Solution::new(&example_inputs_1()).part_2());
        assert_eq!(126, Solution::new(&example_inputs_2()).part_2());
    }
}
