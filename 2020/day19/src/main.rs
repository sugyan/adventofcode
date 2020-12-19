use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{BufRead, BufReader};

struct Solution {
    inputs: Vec<String>,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        Self { inputs }
    }
    fn solve_1(&self) -> usize {
        let mut dict: HashMap<usize, Vec<String>> = HashMap::new();
        let mut rules: HashMap<usize, Vec<Vec<usize>>> = HashMap::new();
        let mut messages: Vec<&String> = Vec::new();
        for input in self.inputs.iter() {
            if input.is_empty() {
                continue;
            }
            if input.starts_with(|c: char| c.is_numeric()) {
                let s: Vec<&str> = input.split(": ").collect();
                if let Ok(key) = s[0].parse() {
                    if s[1].starts_with('"') {
                        dict.insert(key, vec![s[1][1..2].to_string()]);
                    } else {
                        rules.insert(
                            key,
                            s[1].split(" | ")
                                .map(|s| s.split(' ').filter_map(|s| s.parse().ok()).collect())
                                .collect(),
                        );
                    }
                }
            } else {
                messages.push(input);
            }
        }
        while !rules.is_empty() {
            let mut delete: Vec<usize> = Vec::new();
            for (&k, v) in rules.iter() {
                if v.iter().all(|v| v.iter().all(|i| dict.contains_key(i))) {
                    let mut entries: Vec<String> = Vec::new();
                    for v in v.iter() {
                        let mut vd: VecDeque<String> = VecDeque::new();
                        vd.push_back(String::new());
                        for &i in v.iter() {
                            if let Some(values) = dict.get(&i) {
                                for _ in 0..vd.len() {
                                    if let Some(front) = vd.pop_front() {
                                        for s in values.iter() {
                                            vd.push_back(front.clone() + s);
                                        }
                                    }
                                }
                            }
                        }
                        entries.extend(vd.into_iter());
                    }
                    dict.insert(k, entries);
                    delete.push(k);
                }
            }
            for i in delete.iter() {
                rules.remove(i);
            }
        }
        let mut valid: HashSet<&String> = HashSet::new();
        if let Some(v) = dict.get(&0) {
            valid.extend(v.iter());
        };
        messages.iter().filter(|&m| valid.contains(m)).count()
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
            2,
            Solution::new(
                r#"
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#[1..]
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect()
            )
            .solve_1()
        );
    }
}
