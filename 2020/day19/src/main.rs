use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{BufRead, BufReader};

struct Solution {
    valid: HashMap<usize, HashSet<String>>, // inputs: Vec<String>,
    messages: Vec<String>,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        let mut valid: HashMap<usize, HashSet<String>> = HashMap::new();
        let mut rules: HashMap<usize, Vec<Vec<usize>>> = HashMap::new();
        let mut messages: Vec<String> = Vec::new();
        for input in inputs.iter().filter(|&s| !s.is_empty()) {
            if input.starts_with(|c: char| c.is_numeric()) {
                let s: Vec<&str> = input.split(": ").collect();
                if let Ok(key) = s[0].parse() {
                    if s[1].starts_with('"') {
                        let mut hs = HashSet::new();
                        hs.insert(s[1][1..2].to_string());
                        valid.insert(key, hs);
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
                messages.push(input.to_string());
            }
        }
        Solution::expand(&mut rules, &mut valid);
        Self { valid, messages }
    }
    fn expand(
        rules: &mut HashMap<usize, Vec<Vec<usize>>>,
        valid: &mut HashMap<usize, HashSet<String>>,
    ) {
        while !rules.is_empty() {
            let mut delete: Vec<usize> = Vec::new();
            for (&k, v) in rules.iter() {
                if v.iter().all(|v| v.iter().all(|i| valid.contains_key(i))) {
                    let mut entries: HashSet<String> = HashSet::new();
                    for v in v.iter() {
                        let mut vd: VecDeque<String> = VecDeque::new();
                        vd.push_back(String::new());
                        for &i in v.iter() {
                            if let Some(values) = valid.get(&i) {
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
                    valid.insert(k, entries);
                    delete.push(k);
                }
            }
            for i in delete.iter() {
                rules.remove(i);
            }
        }
    }
    fn solve_1(&self) -> usize {
        if let Some(valid) = self.valid.get(&0) {
            self.messages.iter().filter(|&m| valid.contains(m)).count()
        } else {
            0
        }
    }
    fn solve_2(&self) -> usize {
        let valid42 = self.valid.get(&42).unwrap();
        let valid31 = self.valid.get(&31).unwrap();
        let l42 = valid42.iter().next().unwrap().len();
        let l31 = valid31.iter().next().unwrap().len();
        self.messages
            .iter()
            .filter(|&m| {
                for i in 2..m.len() / l42 {
                    let (m42, m31) = (&m[..i * l42], &m[i * l42..]);
                    if m31.is_empty()
                        || m42.len() % l42 != 0
                        || m31.len() % l31 != 0
                        || m42.len() / l42 <= m31.len() / l31
                    {
                        continue;
                    }
                    if (0..i).all(|j| valid42.contains(&m42[j * l42..(j + 1) * l42]))
                        && (0..m31.len() / l31)
                            .all(|j| valid31.contains(&m31[j * l31..(j + 1) * l31]))
                    {
                        return true;
                    }
                }
                false
            })
            .count()
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

    #[test]
    fn example_2() {
        let solution = Solution::new(
            r#"
42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#[1..]
                .split('\n')
                .map(|s| s.to_string())
                .collect(),
        );
        assert_eq!(3, solution.solve_1());
        assert_eq!(12, solution.solve_2());
    }
}
